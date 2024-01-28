use actix_web::{get, Responder, web};
use actix_web::error::JsonPayloadError;
use actix_web::web::{Data, Json};
use log::info;
use crate::handlers::config_handler::model::ConfigHandler;

#[get("/config")]
async fn system(config: Data<ConfigHandler>) -> impl Responder{
    info!("Fetched config: {}", &config.to_string());
    Ok::<Json<Data<ConfigHandler>>,JsonPayloadError>(Json(config))
}

pub fn init_ch_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(system);
}