use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub int_type: String,
    pub module: u32,
    pub number: u32,
    pub ip_address: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct InterfaceDTO {
    pub ip_address: String,
    pub mask: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub enum InterfaceStatus {
    UP,
    DOWN,
}