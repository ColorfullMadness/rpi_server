mod not_found;
mod config_handler;
mod network_devices_handler;
mod network_device;

use actix_web::http::header::ContentType;
use std::{env, fs, u32};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use actix_web::{get, HttpServer, App, web, Responder, post, HttpResponse, ResponseError, put, delete};
use actix_web::error::{JsonPayloadError, PayloadError};
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::web::{Data, Json};
use env_logger::Env;
use handlebars::Handlebars;
use log::info;
use serde::Serialize;
use crate::config_handler::ConfigHandler;
use crate::network_device::{NetworkDevice, Vlan, VlanDTO, InterfaceDTO, Interface};
use crate::network_devices_handler::NetworkDevicesHandler;

use derive_more::{Error, Display};
#[derive(Debug, Display, Error)]
pub struct ExecutionError {
    message: String,
}
impl ResponseError for ExecutionError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::BAD_REQUEST)
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

#[get("/status/health")]
async fn health() -> impl Responder {
    HttpResponse::new(StatusCode::NO_CONTENT)
}

#[get("/config")]
async fn system(config: Data<ConfigHandler>) -> impl Responder{
    info!("Fetched config: {}", &config.to_string());
    Ok::<Json<Data<ConfigHandler>>,JsonPayloadError>(Json(config))
}

#[get("/devices")]
async fn network_devices(devices: Data<Mutex<NetworkDevicesHandler>>) -> impl Responder {
    let devices_data = &devices.lock().unwrap().devices;
    Ok::<Json<HashMap<u32,NetworkDevice>>,JsonPayloadError>(Json(devices_data.clone()))
}

#[get("/device/{id}")]
async fn get_network_device(path: web::Path<u32>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<Json<NetworkDevice>, ExecutionError>{
    let id = path.into_inner();
    let devices = &network_devices_handler.lock().unwrap().devices;
    let device_u =devices.get_key_value(&id);
    match device_u {
        None => Err(ExecutionError { message:"Couldn't find device".to_string() }),
        Some((_, device)) => Ok(Json(device.clone()))
    }
}

#[delete("/device/{id}/vlan/{vlan_id}")]
async fn delete_vlan_from_device(path: web::Path<(u32, u32)> ,devices: Data<Mutex<NetworkDevicesHandler>>) -> Result<String, ExecutionError> {
    let (device_id, vlan_id) = path.into_inner();
    let devices = &mut devices.lock().unwrap().devices;
    let device = devices.get_mut(&device_id);
    match device {
        None => Err(ExecutionError { message:"Couldn't find device".to_string() }),
        Some((device)) => {
            device.remove_vlan(vlan_id)
        }
    }
}

#[post("/device/{id}/hostname/{hostname}")]
async fn change_device_hostname(path: web::Path<(u32, String)>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<Json<NetworkDevice>, ExecutionError>{
    let (id, hostname) = path.into_inner();

    match network_devices_handler.lock().unwrap().devices.get_mut(&id) {
        None => {
            Err(ExecutionError { message:"Couldn't find device.".to_string()})
        }
        Some(device) => {
            match device.execute_command(("en \nconf t \nhostname".to_owned() + &*hostname + "\n").as_str()) {
                Ok(response) => {
                    device.hostname = hostname.to_string();
                    Ok(Json(device.clone()))
                }
                Err(why) => {
                    Err(ExecutionError { message: "Couldn't change hostname".to_string()})
                }
            }
        }
    }
}

#[post("/device/{device_id}/vlan")]
async fn add_vlan(path: web::Path<u32>, vlan_dto: Json<VlanDTO>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<String, ExecutionError> {
    let id = path.into_inner();

    match network_devices_handler.lock().unwrap().devices.get_mut(&id) {
        None => {
                Err(ExecutionError { message:"Couldn't find device.".to_string()})
        }
        Some(device) => {
            device.add_vlan(vlan_dto.into_inner())
        }
    }
}

#[post("/device/{device_id}/interface/{interface_id}")]
async fn conf_interface(path: web::Path<(u32, u32)>, interface_dto: Json<InterfaceDTO>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<Json<NetworkDevice>, ExecutionError> {
    let (device_id, interface_id) = path.into_inner();

    match network_devices_handler.lock().unwrap().devices.get_mut(&device_id) {
        None => {
            Err(
                ExecutionError{
                    message: "Couldn't find device.".to_string()
                }
            )
        }
        Some(device) => {
            match device.interfaces.get_mut(&interface_id) {
                None => {
                    Err(
                        ExecutionError{
                            message: "Couldn't find interface.".to_string()
                        }
                    )
                }
                Some(interface) => {
                    let statuss = match interface_dto.status {
                        "up" => {
                            "no shutdown"
                        },
                        "down" => {
                            "shutdown"
                        }
                        _ => {""}
                    };
                    device.execute_command(&format!("en \n conf t \n ip route {} {} {} \n {}", interface_dto.ip_address_from, interface_dto.mask_from, interface_dto.ip_address_to, statuss));
                    Ok(Json(device.read_interfaces()?.clone()))
                }
            }
        }
    }

}

#[get("/device/{id}/reload_configs")]
async fn reload_configs(path: web::Path<u32>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<Json<NetworkDevice>, ExecutionError> {
    let id = path.into_inner();

    match network_devices_handler.lock().unwrap().devices.get_mut(&id) {
        None => {
            Err(ExecutionError {
                message: "Couldn't find device.".to_string()
            })
        },
        Some(device) => {
            device.read_running_config().expect("Couldn't read running config");
            device.read_startup_config().expect("Couldn't read startup config");
            Ok(Json(device.clone()))
        }
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env::set_var("RUST_LOG", "main,main=config_handler,main=NetworkDevicesHandler,actix_web=debug,actix_server=info");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    info!("Starting");
    println!("{:?}", env::current_dir());

    let mut devices_handler = NetworkDevicesHandler::default();
    devices_handler.read_interfaces();
    devices_handler.read_vlans();
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
            .app_data(Data::new(Mutex::new(devices_handler.clone())))
            .wrap(Logger::default())
            .service(health)
            .service(system)
            .service(network_devices)
            .service(get_network_device)
            .service(change_device_hostname)
            .service(delete_vlan_from_device)
            .service(reload_configs)
            .default_service(web::route().to(not_found::not_found))
    })
        .bind((ip_addr.to_owned(), 8080))?
        .run()
        .await
}