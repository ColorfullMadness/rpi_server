use actix_web::{http, HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Data;
use handlebars::Handlebars;
use serde_json::json;
pub async fn not_found(req: HttpRequest, handlebars: Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({ "url": req.uri().to_string() });
    let body = handlebars.render("not-found", &data).expect("Couldn't render!");

    HttpResponse::Ok()
        .status(http::StatusCode::NOT_FOUND)
        .body(body)
}