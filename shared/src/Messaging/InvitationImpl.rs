use std::io::Error;
use uuid::Uuid;

use super::{Invitation};

impl Invitation {
    // creates a blank invitation, details to be updated by consumer
    pub fn new() -> Invitation {
        Invitation {
            id:  Uuid::new_v4().to_string(),
            recipient_keys: vec![],
            type_field: "https://didcomm.org/connections/1.0/invitation".to_string(),
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
