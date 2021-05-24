use std::io::Error;
use uuid::Uuid;

use crate::Wallets::Records::{
    ConnectionRecord,
    ConnectionState
};
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

    pub fn as_connection_record(&self, state: ConnectionState) -> ConnectionRecord {
        ConnectionRecord {
            id: self.connection_id.to_string(),
            did: self.invitation.did.to_string(),
            image_url: "".to_string(),
            label: self.invitation.image_url.to_string(),
            recipient_keys: self.invitation.recipient_keys.clone(),
            routing_keys: self.invitation.routing_keys.clone(),
            service_endpoint: self.invitation.service_endpoint.to_string(),
            state,
        }
    }
}