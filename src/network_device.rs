use serde::{Deserialize, Serialize};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct NetworkDevice {
    pub ip_address: String,
    pub s_port: String,
    pub manufacturer: String,
    pub hostname: String,
}

impl Default for NetworkDevice {
    fn default() -> Self {
        NetworkDevice {
            ip_address: "0.0.0.0".to_string(),
            s_port: "COM69".to_string(),
            manufacturer: "Cisco".to_string(),
            hostname: "Router".to_string(),
        }
    }
}