use std::collections::HashMap;
use std::io::Read;
use std::net::IpAddr;
use std::thread::panicking;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use serial2::SerialPort;
use substring::Substring;
use crate::ExecutionError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDevice {
    pub serial_number: String, 
    pub ip_address: String,
    pub s_port: String,
    pub hostname: String,
    pub vlans: HashMap<u32, Vlan>,
    pub interfaces: HashMap<u32, Interface>,
    pub startup_config: String,
    pub running_config: String,
}

impl Default for NetworkDevice {
    fn default() -> Self {
        NetworkDevice {
            serial_number: "".to_string(),
            ip_address: "0.0.0.0".to_string(),
            s_port: "COM69".to_string(),
            hostname: "Router".to_string(),
            vlans: HashMap::new(),
            interfaces: HashMap::new(),
            startup_config: "".to_string(),
            running_config: "".to_string(),
        }
    }
}

impl NetworkDevice {
    pub fn execute_command(&self, command:&str) -> Result<String, std::io::Error> {
        let mut response = &mut "".to_string();
        match SerialPort::open(self.s_port.clone(), 9600) {
            Ok(mut port) => {
                match port.write(command.as_ref()) {
                    Ok(res) => {
                        port.read_to_string(response);
                        println!("{}",response);
                        return Ok(response.clone());
                    }
                    Err(why) => return Err(why)
                }
            }
            Err(why) => return Err(why)
        }
    }

    pub fn read_running_config(&mut self) -> Result<String, ExecutionError> {
        return match self.execute_command("sh running-config") {
            Err(why) => {
                Err(ExecutionError {
                    message: why.to_string()
                })
            }
            Ok(result) => {
                self.running_config = result.lines().skip(1).collect();
                Ok("Successfully read running-config".to_string())
            }
        }
    }

    pub fn read_startup_config(&mut self) -> Result<String, ExecutionError> {
        return match self.execute_command("sh startup-config") {
            Err(why) => {
                Err(ExecutionError {
                    message: why.to_string()
                })
            }
            Ok(result) => {
                self.startup_config = result.lines().skip(1).collect();
                Ok("Successfully read startup-config".to_string())
            }
        }
    }

    pub fn set_vlans(&mut self, vlans: HashMap<u32, Vlan>) {
        self.vlans = vlans;
    }

    //TODO trzeba będzie dodać en do wszystkich komend, najlepiej w funkcji wykonującej komendy przy przerobieniu jej na
    // działanie na pojedyńczym połączeniu na urządzenie
    pub fn read_vlans(&mut self) {
        match self.execute_command("sh vlan brief") {
            Ok(response) => {
                let mut name_index:usize = 0;
                let mut status_index:usize = 0;
                let mut ports_index:usize = 0;

                let mut vlans = HashMap::new();
                let mut last_vlan: u32 = 0;
                response.lines().skip(1).for_each(|line| {
                    if line.starts_with("VLAN") {
                        name_index = line.find("Name").unwrap_or(5);
                        status_index = line.find("Status").unwrap_or(38);
                        ports_index = line.find("Ports").unwrap_or(48);
                    } else if (line.starts_with("-")){

                    } else {
                        let nr = line.substring(0,name_index.checked_sub(1).unwrap_or(0)).trim().parse::<u32>();
                        match nr {
                            Ok(nr) => {
                                let name = line.substring(name_index, status_index-1).trim();
                                let status = line.substring(status_index, ports_index-1).trim();
                                let ports = line.substring(ports_index, line.len()).trim();

                                let ports_p: Vec<u32> = parse_interfaces(self, ports);

                                let vlan = Vlan {
                                    name: name.to_string(),
                                    status: status.to_string(),
                                    ports: ports_p,
                                };
                                vlans.insert(nr,vlan);
                                last_vlan = nr;
                            }
                            Err(why) => {
                                let mut vlan = vlans.get_mut(&last_vlan).expect("Should be vlan present here.");
                                let ports = line.substring(ports_index, line.len()).trim();

                                let mut ports_p: Vec<u32> = parse_interfaces(self, ports);
                                vlan.ports.append(&mut ports_p);
                            }
                        }
                    }
                });
                self.set_vlans(vlans);
            }
            Err(why) => {
                println!("Couldn't read response from device {:?} because {}",self,why)
            }
        }
    }

    pub fn remove_vlan(&mut self, vlan_id: u32) -> Result<String, ExecutionError> {
        return match self.execute_command(&format!("en\n conf t\n no vlan {}", vlan_id)) {
            Err(why) => {
                Err(ExecutionError {
                   message: why.to_string()
                })
            }
            Ok(_) => {
                self.read_vlans();
                Ok(String::from("Successfully deleted vlan."))
            }
        }
    }

    pub fn add_vlan(&mut self, vlan: VlanDTO) -> Result<String, ExecutionError>{
        return match self.execute_command(&format!("en\n conf t\n vlan {}", vlan.number)) {
            Ok(_response) => {
                match self.execute_command(&format!("name {}", vlan.name)) {
                    Ok(_response) => {
                        self.read_vlans();
                        Ok(String::from("Successfully added vlan."))
                    }
                    Err(why) => {
                        Err(
                            ExecutionError { message: why.to_string()}
                        )
                    }
                }
            }
            Err(why) => {
                Err(
                    ExecutionError { message: why.to_string()}
                )
            }
        }
    }

    pub fn read_interfaces(&mut self) -> Result<&mut NetworkDevice, ExecutionError> {
        match self.execute_command("sh ip int brief") {
            Ok(response) => {
                let mut interface_index:usize = 0;
                let mut ip_address_index:usize = 0;
                let mut ok_index:usize = 0;
                let mut method_index:usize = 0;
                let mut status_index:usize = 0;
                let mut protocol_index:usize = 0;

                let mut ports:HashMap<u32, Interface> = HashMap::new();
                
                response.lines().skip(1).enumerate().for_each(|(nr, line)| {
                    if line.starts_with("Interface") {
                        interface_index = line.find("Interface").unwrap_or(5);
                        ip_address_index = line.find("IP-Address").unwrap_or(38);
                        ok_index = line.find("OK?").unwrap_or(48);
                        method_index = line.find("Method").unwrap_or(48);
                        status_index = line.find("Status").unwrap_or(48);
                        protocol_index = line.find("Protocol").unwrap_or(48);
                    } else {
                        let port = line.substring(interface_index, ip_address_index - 1).trim();
                        let ip_address = line.substring(ip_address_index, ok_index - 1).trim();
                        let ok = line.substring(ok_index, method_index - 1).trim();
                        let method = line.substring(method_index, status_index - 1).trim();
                        let status = line.substring(status_index, protocol_index - 1).trim();
                        let protocol = line.substring(protocol_index, line.len()).trim();

                        let pat = &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
                        let first_digit = port.find(pat).unwrap();
                        println!("First digit: {}, coresponging leter: {:?}", first_digit, port.get(first_digit..));
                        let (interface, port) = port.split_at(first_digit);
                        println!("Interface: {}",interface);
                        let (module,port_nr) = port.split_once("/").unwrap_or((port, ""));
                        println!("Module: {}, Port_number: {}", module, port_nr);

                        let port = Interface{
                            int_type: interface.to_string(),
                            module: module.parse().unwrap(),
                            number: port_nr.parse().unwrap_or(0),
                            ip_address: ip_address.parse().unwrap(),
                            status: status.parse().unwrap(),
                        };

                        ports.insert(nr as u32, port);
                    }
                });
                self.interfaces = ports;
                Ok(self)
            }
            Err(why) => {
                println!("Couldn't read response from device {:?} because {}",self,why);
                Err(ExecutionError{
                    message: "Couldn't find device".to_string()
                })
            }
        }
    }

    //TODO
    // fn add_int_to_vlan(&mut self, vlan: VlanDTO) -> Result<String, MyError> {
    //     return match
    // }
}

fn parse_interfaces(device: &NetworkDevice, ports: &str) -> Vec<u32> {
    if ports.is_empty() {
        return Vec::new();
    }
    //TODO to nie dziala dla portow w formacie 3/13-16, sprawdzic co to za format portow czy ejst konieczne wspieranie
    ports.split(",").map(|port|{
        let (kind, port_mod): (&str, &str) = port.trim().split_at(2);
        let (port_mod_p, port_nr): (&str, &str) = port_mod.split_once("/").expect("Mod/Port format not correct.");

        let kind_m = match kind {
            "Fa" => "FastEthernet",
            "Ga" => "GigabitEthernet",
            _ => ""
        };

        println!("Interfaces: {:?}", device.interfaces);

        device.interfaces.iter()
            .filter(|(id, int)|{
                int.int_type == kind_m && int.module == port_mod_p.parse::<u32>().unwrap() && int.number == port_nr.parse::<u32>().unwrap()
            })
            .map(|(id, int)|{
                println!("ID: {}", id);
                id.clone()
            })
            .last()
            .unwrap()
    }).collect()
}

#[derive(Debug,Deserialize)]
pub struct VlanDTO {
    pub number: u32,
    pub name: String,
    pub interfaces: Vec<u32>
}

#[derive(Debug, Deserialize)]
pub enum InterfaceStatus {
    UP,
    DOWN,
}
#[derive(Debug, Deserialize)]
pub struct InterfaceDTO {
    pub ip_address_from: String,
    pub mask_from: String,
    pub ip_address_to: String,
    pub status: String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Vlan {
    pub(crate) name: String,
    pub(crate) status: String, //TODO change this to enum with possible status values
    pub(crate) ports: Vec<u32> //TODO najpierw trzeba zaczytać interfejsy i potem można przypisać ich idki do vlanów
}

impl Default for Vlan {
    fn default() -> Self {
        Vlan {
            name: "Vlan1".to_string(),
            status: "Suspended".to_string(),
            ports: Vec::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub int_type: String,
    pub module: u32,
    pub number: u32,
    pub ip_address: String,
    pub status: String,
}
