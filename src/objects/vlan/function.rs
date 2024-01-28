use super::model::Vlan;

impl Default for Vlan {
    fn default() -> Self {
        Vlan {
            name: "Vlan1".to_string(),
            status: "Suspended".to_string(),
            ports: Vec::new()
        }
    }
}