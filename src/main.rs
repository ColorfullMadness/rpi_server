mod not_found;
mod config_handler;

use std::{env, fs};
use actix_web::{get, HttpServer, App, web, Responder, post, HttpResponse};
use actix_web::error::JsonPayloadError;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlerResponse::Response;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;
use handlebars::Handlebars;
use log::info;
use serde::Serialize;
use crate::config_handler::ConfigHandler;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!\n", name)
}

#[get("/status/health")]
async fn health() -> impl Responder {
    // let health = Health {
    //     status: "Ok".to_string(),
    // };
    // Ok::<web::Json<Health>, JsonPayloadError>(web::Json(health))
    HttpResponse::new(StatusCode::NO_CONTENT)
}

#[get("/status/config")]
async fn system(config: web::Data<ConfigHandler>) -> impl Responder{
    info!("Fetched config: {}", &config.to_string());
    Ok::<web::Json<Data<ConfigHandler>>,JsonPayloadError>(web::Json(config))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env::set_var("RUST_LOG", "config_handler=info, main=info ,actix_web=debug,actix_server=info");

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    info!("Starting");
    println!("{:?}", env::current_dir());

    let mut conf = ConfigHandler::init(&Default::default());
    let templates = conf.templates_loc.clone();
    let ip_addr = conf.ip_address.clone();

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", templates).expect("Couldn't load templates");

    let data = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .app_data(web::Data::new(conf.clone()))
            .wrap(Logger::default())
            .service(greet)
            .service(health)
            .service(system)
            .default_service(web::route().to(not_found::not_found))
    })
        .bind((ip_addr.to_owned(), 8080))?
        .run()
        .await
}