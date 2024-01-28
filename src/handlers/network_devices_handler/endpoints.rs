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
async fn get_network_devices(devices: Data<Mutex<NetworkDevicesHandler>>) -> impl Responder {
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
async fn delete_vlan(path: web::Path<(u32, u32)> ,devices: Data<Mutex<NetworkDevicesHandler>>) -> Result<String, ExecutionError> {
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
async fn change_hostname(path: web::Path<(u32, String)>, network_devices_handler: Data<Mutex<NetworkDevicesHandler>>) -> Result<Json<NetworkDevice>, ExecutionError>{
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
                    let statuss = match interface_dto.status.as_ref() {
                        "up" => {
                            "no shutdown"
                        },
                        "down" => {
                            "shutdown"
                        }
                        _ => {""}
                    };
                    device.execute_command(&format!("en \n conf t \n ip route {} {} {} \n {}", interface_dto.ip_address_from, interface_dto.mask_from, interface_dto.ip_address_to, statuss)).expect("TODO: panic message");
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

pub fn init_nd_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(get_network_devices);
    cfg.service(get_network_device);
    cfg.service(change_hostname);
    cfg.service(add_vlan);
    cfg.service(delete_vlan);
    cfg.service(conf_interface);
    cfg.service(reload_configs);
}