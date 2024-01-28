use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigHandler {
    pub uuid: String,
    pub ip_address: String,
    pub platform: String,
    pub templates_loc: String,
    pub mac_address: String,
    pub version: String,
}