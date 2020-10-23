use std::io::Error;
use uuid::Uuid;

use super::{CreateInvitationResponse, Invitation};

impl CreateInvitationResponse {
    // creates a blank CreateInvitationResponse, details to be updated by consumer
    pub fn new() -> CreateInvitationResponse {
        CreateInvitationResponse {
            connection_id :  Uuid::new_v4().to_string(),
            invitation_url: "".to_string(),
            invitation: Invitation::new()
        }
    }

    pub fn from_json(json: &str) -> Result<CreateInvitationResponse, Error> {
        let message: CreateInvitationResponse = serde_json::from_str(json)?;
        Ok(message)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}