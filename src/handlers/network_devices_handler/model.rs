use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::objects::device::model::NetworkDevice;

#[derive(Clone,Serialize,Deserialize)]
pub struct NetworkDevicesHandler {
    pub devices: HashMap<u32, NetworkDevice>,
}