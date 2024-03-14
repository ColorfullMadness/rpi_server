use std::collections::HashMap;
use std::io::Read;
use serial2::SerialPort;
use crate::errors::execution_error::ExecutionError;

use super::model::NetworkDevicesHandler;
use crate::objects::device::model::NetworkDevice;
use crate::objects::interface::model::InterfaceDTO;
use crate::objects::vlan::model::VlanDTO;

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
                        Ok(_res) => {
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
                                    Err(_why) => {}
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
                                next_id += 1;
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

    pub fn get_device(&mut self, id: u32) -> Result<&mut NetworkDevice, ExecutionError> {
        match self.devices.get_mut(&id) {
            Some(device) => Ok(device),
            None => Err(ExecutionError{message: "Couldnt find device.".to_string()})
        }
    }

    pub fn add_vlan(&mut self, id: u32, vlan: VlanDTO) -> Result<String, ExecutionError> {
        let mut device = self.get_device(id)?;
        device.add_vlan(vlan)
    }

    pub fn remove_vlan(&mut self, device_id: u32, vlan_id: u32) -> Result<String, ExecutionError> {
        let mut device = self.get_device(device_id)?;
        device.remove_vlan(vlan_id)
    }

    pub fn change_hostname(&mut self, device_id: u32, hostname: &str) -> Result<&NetworkDevice, ExecutionError> {
        let mut device = self.get_device(device_id)?;
        match device.execute_command(("en \nconf t \nhostname".to_owned() + &*hostname + "\n").as_str()) {
            Ok(_response) => {
                device.hostname = hostname.to_string();
                Ok(device)
            }
            Err(_why) => { //This error should probably be piped to some kind of per device error handling for case when it stops working mid session
                Err(ExecutionError { message: "Couldn't change hostname".to_string()})
            }
        }
    }

    pub fn configure_interface(&mut self, device_id: u32, interface_id: u32, interface_dto: InterfaceDTO) -> Result<&mut NetworkDevice, ExecutionError> {
        let device = self.get_device(device_id)?;
        device.configure_interface(interface_id, interface_dto)
    }

    pub fn reload_configs(&mut self, device_id: u32) -> Result<&NetworkDevice, ExecutionError> {
        let mut device = self.get_device(device_id)?;

        device.read_running_config()?;
        device.read_startup_config()?;

        Ok(device)
    }
}