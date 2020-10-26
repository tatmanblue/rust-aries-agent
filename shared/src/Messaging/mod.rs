// Protocol messages are data structures shared between agents as defined in the aries RFCs.

#![allow(non_snake_case)]
use chrono::DateTime;
use chrono::Utc;
use crate::Locale::Locales;

// public mods.  These are visible to consumers
pub mod Enums;
pub mod Parameters;

// private mods, nothing exported out of Shared library in these mods
mod BasicMessageImpl;
mod CreateInvitationResponseImpl;
mod InvitationImpl;

// -----------------------------------------------------------------------------------------------
// This file is likely to get long.  Keep all structures organized alphabetically.  It contains
// of the message structures for all aries protocols implemented
// see bottom section for small exception
// -----------------------------------------------------------------------------------------------

// Expected input for BasicMessage input, RFC '0095-basic-message'
//
//
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicMessage {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "~l10n")]
    pub l10n: L10N,
    #[serde(rename = "sent_time")]
    pub sent_time: DateTime<Utc>,
    pub content: String,
}

// Expected output CreateInvitation from the inviter, RFC '0160: Connection Protocol'
//
//
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvitationResponse {
    #[serde(rename = "connection_id")]
    pub connection_id: String,
    #[serde(rename = "invitation_url")]
    pub invitation_url: String,
    pub invitation: Invitation,
}

// Invitation which can be exchanged between inviter and invitee, RFC '0160: Connection Protocol'
//
//
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    #[serde(rename = "@id")]
    pub id: String,
    pub recipient_keys: Vec<String>,
    #[serde(rename = "@type")]
    pub type_field: String,
    pub service_endpoint: String,
    pub image_url: String,
    pub routing_keys: Vec<String>,
    pub did: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L10N {
    pub locale: Locales,
}

// -----------------------------------------------------------------------------------------------
// These types are not tied specifically to an ARIES RFC
// -----------------------------------------------------------------------------------------------

// toThink(): this is not a protocol message but a message
// between rust libraries (shared->host etc).  Prob lives
// elsewhere
// TODO: define what a ErrorResponse means
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub id: u32
}

// toThink(): this is not a protocol message but a message
// between rust libraries (shared->host etc).  Prob lives 
// elsewhere
// TODO: define what a GenericResponse means
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericResponse {
    pub id: u32
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    pub message: String
}

impl StatusResponse {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}