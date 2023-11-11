use std::fmt::{Display, Formatter, write};
use serde::Serialize;
use sysinfo::{System, SystemExt, NetworkExt, Networks, NetworksExt};
use log::info;
use uuid;
#[derive(Serialize)]
pub struct ConfigHandler {
    pub ip_address: String,
    pub platform: String,
    pub templates_loc: String,
    pub mac_address: String,
    pub uuid: String
}

impl Display for ConfigHandler{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("Config: ip_address: {}, mac_address: {}, uuid: {}",
            self.ip_address,
            self.mac_address,
            self.uuid))
    }
}

impl Default for ConfigHandler {
    fn default() -> Self {
        //TODO this should be read from conf file
        let interface_name = "vEthernet (Default Switch)";

        let mut sys = System::new_all();
        sys.refresh_all();

        let mut templates_loc = "./src/templates".to_string();
        let mut address = "127.0.0.1".to_string();
        let networks = sys.networks();

        let mac_address = networks.iter()
            .filter(|(&ref name, &ref data)| name.eq(interface_name))
            .map(|(&ref name, &ref data)| data.mac_address().to_string())
            .collect();
        info!("Mac address: {}", mac_address);

        match sys.name() {
            Some(x) => {
                if x.eq("Raspberry Pi") {
                    templates_loc = "./templates".to_string();
                    address = "10.0.10.5".to_string();
                }
            }
            None => {}
        }

        ConfigHandler {
            ip_address: address,
            platform: sys.name().expect("Couldn't get OS name."),
            templates_loc,
            mac_address,
            uuid: uuid::Uuid::new_v4().to_string(),
        }
    }
}

