#[derive(Debug, Clone)]
pub struct NetworkDevice {
    pub ip_address: String,
    pub s_port: String,
    pub manufacturer: String,
}

impl Default for NetworkDevice {
    fn default() -> Self {
        NetworkDevice {
            ip_address: "0.0.0.0".to_string(),
            s_port: "COM69".to_string(),
            manufacturer: "Cisco".to_string(),
        }
    }
}