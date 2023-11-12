use std::io::Read;
use log::info;
use serial2::SerialPort;
use crate::network_device::NetworkDevice;

#[derive(Clone)]
pub struct NetworkDevicesHandler {
    pub devices: Vec<NetworkDevice>
}

impl Default for NetworkDevicesHandler {
    fn default() -> Self {
        let ports = SerialPort::available_ports().expect("Couldn't read Serial Port list");
        for p in ports {
            println!("Port: {:?}", p);
            match SerialPort::open(p, 9600) {
                Ok(mut port) => {
                    match port.write("show version\n".as_ref()) {
                        Ok(res) => {
                            let response = &mut "".to_string();
                            match port.read_to_string(response) {
                                Ok(_) => {}
                                Err(why) => println!("Couldn't read: {}", why)
                            }
                            println!("{}",response);
                        }
                        Err(why) => println!("Couldn't write: {}", why)
                    }
                }
                Err(why) => println!("Couldn't open: {}", why)
            }
        }

        NetworkDevicesHandler{
            devices: vec![NetworkDevice::default()],
        }
    }
}
impl NetworkDevicesHandler {
    pub fn get_ports() {
    }
}