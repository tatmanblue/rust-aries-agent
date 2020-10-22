use std::io::Error;

use super::{Invitation};

impl Invitation {
    pub fn new() -> Invitation {
        Invitation {
            id: "".to_string(),
            recipient_keys: vec![],
            type_field: "".to_string(),
            service_endpoint: "".to_string(),
            image_url: "".to_string(),
            routing_keys: vec![],
            did: "".to_string(),
            label: "".to_string()
        }
    }

    pub fn from_json(json: &str) -> Result<Invitation, Error> {
        let message: Invitation = serde_json::from_str(json)?;
        Ok(message)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
