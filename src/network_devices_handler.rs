use std::io::Read;
use log::info;
use serde::{Deserialize, Serialize};
use serial2::SerialPort;
use crate::network_device::NetworkDevice;

#[derive(Clone,Serialize,Deserialize)]
pub struct NetworkDevicesHandler {
    pub devices: Vec<NetworkDevice>
}

impl Default for NetworkDevicesHandler {
    fn default() -> Self {
        let ports = SerialPort::available_ports().expect("Couldn't read Serial Port list");
        let response = &mut "".to_string();
        let devices:Vec<NetworkDevice> = Vec::new();
        for p in ports {
            println!("Port: {:?}", p);
            match SerialPort::open(p, 9600) {
                Ok(mut port) => {
                    match port.write("show version\n".as_ref()) {
                        Ok(res) => {
                            match port.read_to_string(response) {
                                Ok(_) => {}
                                Err(why) => println!("Couldn't read: {}", why)
                            }
                            println!("{}",response);
                            if response.contains("Cisco") {
                               //TODO add reading all the important config parts, move reading from port to another function
                            }
                        }
                        Err(why) => println!("Couldn't write: {}", why)
                    }
                }
                Err(why) => println!("Couldn't open: {}", why)
            }
        }

        NetworkDevicesHandler{
            devices: vec![NetworkDevice{
                ip_address: "".to_string(),
                s_port: "".to_string(),
                manufacturer: "".to_string(),
                hostname: response.to_string(),
            }],
        }
    }
}
impl NetworkDevicesHandler {
    pub fn get_ports() {
    }
}