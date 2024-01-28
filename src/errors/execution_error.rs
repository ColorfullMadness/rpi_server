use actix_web::{HttpResponse, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub struct ExecutionError {
    pub message: String,
}
impl ResponseError for ExecutionError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::BAD_REQUEST)
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}