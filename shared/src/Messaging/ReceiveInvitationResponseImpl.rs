use chrono::prelude::*;
use std::io::Error;

use super::{ReceiveInvitationResponse};

impl ReceiveInvitationResponse {
    // creates a blank CreateInvitationResponse, details to be updated by consumer
    pub fn new() -> ReceiveInvitationResponse {
        ReceiveInvitationResponse {
            accept: "".to_string(),
            connection_id :  "".to_string(),
            created_at: Utc::now(),
            invitation_key: "".to_string(),
            invitation_mode: "".to_string(),
            rfc23_state: "".to_string(),
            routing_state: "".to_string(),
            their_role: "".to_string(),
            state: "".to_string(),
            updated_at: Utc::now(),
        }

    }

    pub fn from_json(json: &str) -> Result<ReceiveInvitationResponse, Error> {
        let message: ReceiveInvitationResponse = serde_json::from_str(json)?;
        Ok(message)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Default for ReceiveInvitationResponse {
    fn default() -> Self {
        ReceiveInvitationResponse::new()
    }
}