mod not_found;
mod config_handler;

use std::env;
use actix_web::{get, HttpServer, App, web, Responder, post};
use actix_web::error::JsonPayloadError;
use handlebars::Handlebars;
use serde::Serialize;
use crate::config_handler::ConfigHandler;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!\n", name)
}

#[get("/status")]
async fn health() -> impl Responder {
    let health = Health {
        status: "Ok".to_string(),
    };
    Ok::<web::Json<Health>, JsonPayloadError>(web::Json(health))
}

enum Status {
    OK,
    NotOk
}

#[derive(Serialize)]
struct Health {
    status: String
}

#[get("/status/system")]
async fn system() -> impl Responder{
    let config = ConfigHandler::default();
    Ok::<web::Json<ConfigHandler>,JsonPayloadError>(web::Json(config))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    // let paths = fs::read_dir("./").unwrap();
    //
    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }

    let mut conf = ConfigHandler::default();

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./src/templates/").expect("Couldn't load templates");

    let data = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(greet)
            .service(health)
            .service(system)
            .default_service(web::route().to(not_found::not_found))
    })
        .bind((conf.ip_address, 8080))?
        .run()
        .await
}