use serde::Serialize;
use sysinfo::{System, SystemExt};

#[derive(Serialize)]
pub struct ConfigHandler {
    pub ip_address: String,
    pub platform: String,
    pub templates_loc: String,
}

impl Default for ConfigHandler {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut templates_loc = "./src/templates".to_string();
        let mut address = "127.0.0.1".to_string();
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
            templates_loc
        } 
    }
}

