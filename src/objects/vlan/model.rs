use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize)]
pub struct VlanDTO {
    pub number: u32,
    pub name: String,
    pub interfaces: Vec<u32>
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Vlan {
    pub(crate) name: String,
    pub(crate) status: String, //TODO change this to enum with possible status values
    pub(crate) ports: Vec<u32>
}

