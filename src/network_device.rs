use std::collections::HashMap;
use std::fmt::format;
use std::io::{Error, Read};
use serde::{Deserialize, Serialize};
use serial2::SerialPort;
use substring::Substring;
use crate::MyError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDevice {
    pub ip_address: String,
    pub s_port: String,
    pub hostname: String,
    pub vlans: HashMap<u32, Vlan>,
}

impl Default for NetworkDevice {
    fn default() -> Self {
        NetworkDevice {
            ip_address: "0.0.0.0".to_string(),
            s_port: "COM69".to_string(),
            hostname: "Router".to_string(),
            vlans: HashMap::new(),
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

    pub fn set_vlans(&mut self, vlans: HashMap<u32, Vlan>) {
        self.vlans = vlans;
    }

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

                                let ports_p: Vec<Port> = parse_ports(ports);

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

                                let mut ports_p: Vec<Port> = parse_ports(ports);
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

    pub fn remove_vlan(&mut self, vlan_id: u32) -> Result<String, MyError> {
        return match self.execute_command(&format!("en\n conf t\n no vlan {}", vlan_id)) {
            Err(why) => {
                Err(MyError{
                   name: why.to_string()
                })
            }
            Ok(_) => {
                self.read_vlans();
                Ok(String::from("Successfully deleted vlan."))
            }
        }
    }

    pub fn add_vlan(&mut self, vlan: VlanDTO) -> Result<String, MyError>{
        return match self.execute_command(&format!("en\n conf t\n vlan {}", vlan.number)) {
            Ok(_response) => {
                match self.execute_command(&format!("name {}", vlan.name)) {
                    Ok(_response) => {
                        self.read_vlans();
                        Ok(String::from("Successfully added vlan."))
                    }
                    Err(why) => {
                        Err(
                            MyError{name: why.to_string()}
                        )
                    }
                }
            }
            Err(why) => {
                Err(
                    MyError{name: why.to_string()}
                )
            }
        }
    }
}

fn parse_ports(ports: &str) -> Vec<Port> {
    if ports.is_empty() {
        return Vec::new();
    }
    //TODO to nie dziala dla portow w formacie 3/13-16, sprawdzic co to za format portow czy ejst konieczne wspieranie
    ports.split(",").map(|port|{
        let (interface, port_mod): (&str, &str) = port.trim().split_at(2);
        let (port_mod_p, port_nr): (&str, &str) = port_mod.split_once("/").expect("Mod/Port format not correct.");
        Port {
            interface: interface.to_string(),
            module: port_mod_p.parse().expect(&*("Couldn't parse port module: ".to_owned() + port_mod_p)),
            number: port_nr.parse().expect("Couldn't parse port nr"),
        }
    }).collect()
}

#[derive(Debug,Deserialize)]
pub struct VlanDTO {
    pub number: u32,
    pub name: String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Vlan {
    pub(crate) name: String,
    pub(crate) status: String, //TODO change this to enum with possible status values
    pub(crate) ports: Vec<Port>
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
pub struct Port {
    pub interface: String,
    pub module: u32,
    pub number: u32,
}