use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::objects::interface::model::Interface;
use crate::objects::vlan::model::Vlan;

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