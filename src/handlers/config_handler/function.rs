use std::fmt::{Display, Formatter, write};
use log::info;
use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};
use uuid::Uuid;

use super::model::ConfigHandler;

impl ConfigHandler {
    pub fn init(&self) -> Self {

        let mut sys = System::new_all();
        sys.refresh_all();
        let mut templates_loc = "./src/templates".to_string();
        let mut address = "127.0.0.1".to_string();
        let mut interface_name = "vEthernet (Default Switch)";
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
                    interface_name = "eth0";
                }
            }
            None => {}
        }
        let config:ConfigHandler = ConfigHandler {
            uuid: uuid::Uuid::new_v4().to_string(),
            ip_address: address,
            platform: sys.name().expect("Couldn't get OS name."),
            templates_loc,
            mac_address,
            version: "1.0.0".to_string(),
        };
        return config

        // let path = Path::new("./src/settings.conf");
        // let mut file = match File::open(path) {
        //     Err(why) => {
        //         info!("Couldn't open file {:?}: {}", path, why);
        //         match File::options().write(true).create(true).open(path) {
        //             Err(why) => panic!("Couldn't create settings.conf file, {}", why),
        //             Ok(mut file) => {
        //                 let _ = file.write(serde_json::to_string::<ConfigHandler>(&config).unwrap().as_ref());
        //                 file
        //             }
        //         }
        //     },
        //     Ok(file) => file,
        // };

        // let mut s = String::new();
        // match file.read_to_string(&mut s) {
        //     Err(why) => error!("Couldn't read from file {:?}: {}", path, why),
        //     Ok(_) => info!("Settings file content: {}", s)
        // };
        // serde_json::from_str(&*s).unwrap()
    }
}

impl Default for ConfigHandler{
    fn default() -> Self {
        ConfigHandler{
            uuid: Uuid::new_v4().to_string(),
            ip_address: "127.0.0.1".to_string(),
            platform: "Windows".to_string(),
            templates_loc: "./templates".to_string(),
            mac_address: "00:00:00:00:00".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

impl Display for ConfigHandler{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("Config: ip_address: {}, mac_address: {}, uuid: {}",
                              self.ip_address,
                              self.mac_address,
                              self.uuid))
    }
}

