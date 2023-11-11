mod not_found;
mod config_handler;

use std::env;
use actix_web::{get, HttpServer, App, web, Responder, post, HttpResponse};
use actix_web::error::JsonPayloadError;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlerResponse::Response;
use actix_web::middleware::Logger;
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
async fn system() -> impl Responder{
    let config = ConfigHandler::default();
    info!("Fetched config: {}", config.to_string());
    Ok::<web::Json<ConfigHandler>,JsonPayloadError>(web::Json(config))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env::set_var("RUST_LOG", "config_handler=info, main=info ,actix_web=debug,actix_server=info");

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // let paths = fs::read_dir("./").unwrap();
    //
    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
    info!("Starting");

    let mut conf = ConfigHandler::default();

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", conf.templates_loc).expect("Couldn't load templates");

    let data = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .service(greet)
            .service(health)
            .service(system)
            .default_service(web::route().to(not_found::not_found))
    })
        .bind((conf.ip_address, 8080))?
        .run()
        .await
}