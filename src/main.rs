mod not_found;
mod errors;
mod objects;
mod handlers;

use handlers::network_devices_handler::model::NetworkDevicesHandler;
use handlers::network_devices_handler::endpoints::init_nd_endpoints;
use handlers::config_handler::model::ConfigHandler;
use handlers::config_handler::endpoints::init_ch_endpoints;

use std::{env, fs, u32};
use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{get, HttpServer, App, web, Responder, post, HttpResponse, ResponseError, delete};
use actix_web::error::JsonPayloadError;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::web::{Data, Json};
use dotenv::dotenv;
use env_logger::Env;
use handlebars::Handlebars;
use log::info;
use serde::Serialize;

#[get("/status/health")]
async fn health() -> impl Responder {
    HttpResponse::new(StatusCode::NO_CONTENT)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // dotenv().ok();
    env::set_var("RUST_LOG", "rpi_client=info");
    env_logger::init();

    // let paths = fs::read_dir("./").unwrap();
    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }

    // println!("{:?}", env::current_dir());

    let mut devices_handler = NetworkDevicesHandler::default();
    devices_handler.read_interfaces();
    devices_handler.read_vlans();
    let mut conf = ConfigHandler::init(&Default::default());
    let templates = conf.templates_loc.clone();
    let ip_addr = conf.ip_address.clone();

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", templates).expect("Couldn't load templates");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(handlebars.clone()))
            .app_data(Data::new(conf.clone()))
            .app_data(Data::new(Mutex::new(devices_handler.clone())))
            .wrap(Logger::default())
            .service(health)
            .configure(init_nd_endpoints)
            .configure(init_ch_endpoints)
            .default_service(web::route().to(not_found::not_found))
    })
        .bind((ip_addr.to_owned(), 8080))?
        .run()
        .await
}