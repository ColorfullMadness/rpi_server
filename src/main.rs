mod not_found;

use std::{env, fs};
use actix_web::{main, get, HttpServer, App, web, Responder};
use handlebars::Handlebars;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

// #[get("/json/test1")]
// async fn json() -> impl Responder {
//
// }

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./src/templates/").expect("Couldn't load templates");

    let data = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(greet)
            .default_service(web::route().to(not_found::not_found))
    })
        .bind(("172.27.224.3", 8080))?
        .run()
        .await
}