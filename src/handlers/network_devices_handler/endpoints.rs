use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{delete, get, post, Responder, web};
use actix_web::error::JsonPayloadError;
use actix_web::web::{Data, Json};

use crate::errors::execution_error::ExecutionError;
use crate::handlers::network_devices_handler::model::NetworkDevicesHandler;
use crate::objects::device::model::NetworkDevice;
use crate::objects::interface::model::InterfaceDTO;
use crate::objects::vlan::model::VlanDTO;

#[get("/devices")]
async fn get_network_devices(devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> impl Responder {
    let devices_data = &devices_handler.lock().unwrap().devices;
    Ok::<Json<HashMap<u32,NetworkDevice>>,JsonPayloadError>(Json(devices_data.clone()))
}

#[get("/device/{id}")]
async fn get_network_device(path: web::Path<u32>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<Json<NetworkDevice>, ExecutionError>{
    let id = path.into_inner();
    let mut devices_handler = &network_devices_handler.lock().unwrap();
    let device = devices_handler.get_device(id)?;
    Ok(Json(device.clone()))
}

#[post("/device/{device_id}/vlan")]
async fn add_vlan(path: web::Path<u32>, vlan_dto: Json<VlanDTO>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<String, ExecutionError> {
    let id = path.into_inner();
    let mut devices_handler = network_devices_handler.lock().unwrap();
    devices_handler.add_vlan(id, vlan_dto.into_inner())
}

#[delete("/device/{device_id}/vlan/{vlan_id}")]
async fn delete_vlan(path: web::Path<(u32, u32)> ,network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<String, ExecutionError> {
    let (device_id, vlan_id) = path.into_inner();
    let mut devices_handler = &network_devices_handler.lock().unwrap();
    devices_handler.remove_vlan(device_id, vlan_id)
}

#[post("/device/{id}/hostname/{hostname}")]
async fn change_hostname(path: web::Path<(u32, &str)>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<Json<NetworkDevice>, ExecutionError>{
    let (id, hostname) = path.into_inner();
    let mut devices_handler = network_devices_handler.lock().unwrap();
    let device_conf = devices_handler.change_hostname(id, hostname)?;
    Ok(Json(device_conf.clone()))
}

#[post("/device/{device_id}/interface/{interface_id}")]
async fn conf_interface(path: web::Path<(u32, u32)>, interface_dto: Json<InterfaceDTO>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<Json<NetworkDevice>, ExecutionError> {
    let (device_id, interface_id) = path.into_inner();
    let device = network_devices_handler.lock().unwrap().configure_interface(device_id, interface_id, interface_dto.into_inner())?;

    Ok(Json(device.clone()))
}

#[get("/device/{id}/reload_configs")]
async fn reload_configs(path: web::Path<u32>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<Json<NetworkDevice>, ExecutionError> {
    let id = path.into_inner();
    let mut devices_manager = network_devices_handler.lock().unwrap();
    let device = devices_manager.reload_configs(id)?;

    Ok(Json(device.clone()))
}

pub fn init_nd_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(get_network_devices);
    cfg.service(get_network_device);
    cfg.service(change_hostname);
    cfg.service(add_vlan);
    cfg.service(delete_vlan);
    cfg.service(conf_interface);
    cfg.service(reload_configs);
}