use std::collections::HashMap;
use std::io::Read;
use serial2::SerialPort;

use super::model::NetworkDevicesHandler;
use crate::objects::device::model::NetworkDevice;

impl Default for NetworkDevicesHandler {
    fn default() -> Self {
        let ports = SerialPort::available_ports().expect("Couldn't read Serial Port list");
        let response = &mut "".to_string();
        let mut network_devices:HashMap<u32, NetworkDevice> = HashMap::new();
        let mut next_id = 1;
        for p in ports {
            println!("Port: {:?}", p);
            match SerialPort::open(p.clone(), 9600) {
                Ok(mut port) => {
                    match port.write("show version\n".as_ref()) {
                        Ok(res) => {
                            match port.read_to_string(response) {
                                Ok(_) => {}
                                Err(why) => println!("Couldn't read: {}", why)
                            }
                            println!("{}",response);
                            if response.contains("Cisco") {
                                response.clear();
                                port.write("hostname\n".as_ref()).unwrap();
                                match port.read_to_string(response) {
                                    Ok(_) => {}
                                    Err(why) => {}
                                }
                                let r_s = response.to_string();
                                let output = r_s.split_once("\n").unwrap();
                                network_devices.insert(next_id, NetworkDevice {
                                    serial_number: "".to_string(),
                                    ip_address: "".to_string(),
                                    s_port: p.to_str().unwrap().to_string(),
                                    hostname: output.1.trim().to_string(),
                                    vlans: HashMap::new(),
                                    interfaces: Default::default(),
                                    startup_config: "".to_string(),
                                    running_config: "".to_string(),
                                });
                            }
                        }
                        Err(why) => println!("Couldn't write: {}", why)
                    }
                }
                Err(why) => println!("Couldn't open: {}", why)
            }
        }

        NetworkDevicesHandler{
            devices: network_devices,
        }
    }
}
impl NetworkDevicesHandler {
    pub fn read_vlans(&mut self) {
        for (_, device) in self.devices.iter_mut() {
            device.read_vlans();
        }
    }

    pub fn read_interfaces(&mut self) {
        for (_, device) in self.devices.iter_mut() {
            device.read_interfaces().expect("TODO: panic message");
        }
    }
}
