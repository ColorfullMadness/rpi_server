mod not_found;

use std::env;
use actix_web::{get, HttpServer, App, web, Responder};
use actix_web::error::JsonPayloadError;
use handlebars::Handlebars;
use serde::Serialize;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!\n", name)
}

// #[get("/json/test1")]
// async fn json() -> impl Responder {
//
// }
#[get("/health")]
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

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    // let paths = fs::read_dir("./").unwrap();
    //
    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./src/templates/").expect("Couldn't load templates");

    let data = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(greet)
            .service(health)
            .default_service(web::route().to(not_found::not_found))
    })
        .bind(("10.0.10.5", 8080))?
        .run()
        .await
    // 172.27.224.3
}