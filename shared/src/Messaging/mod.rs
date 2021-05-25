// Protocol messages are data structures shared between agents as defined in the aries RFCs.
// all types in these structs need to compatible across the wire

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
mod ReceiveInvitationResponseImpl;

// -----------------------------------------------------------------------------------------------
// This file is likely to get long.  Keep all structures organized alphabetically.  It contains
// of the message structures for all aries protocols implemented
// see bottom section for small exception
//
// note about field ordering:
// with the exception of 'id' and 'type' fields, all fields should be arranged alphabetically
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
    pub content: String,
    #[serde(rename = "~l10n")]
    pub l10n: L10N,
    #[serde(rename = "sent_time")]
    pub sent_time: DateTime<Utc>,
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
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub did: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub image_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    pub recipient_keys: Vec<String>,
    pub routing_keys: Vec<String>,
    pub service_endpoint: String,
}

/*
{
    "their_role": "inviter",
    "created_at": "2021-05-25 18:23:54.035469Z",
    "invitation_key": "BteySsVyJVdMqMDkAcA9ZDGDEP9TAG6oNfsPFVGotyp1",
    "rfc23_state": "invitation-received",
    "accept": "manual",
    "invitation_mode": "once",
    "routing_state": "none",
    "updated_at": "2021-05-25 18:23:54.035469Z",
    "state": "invitation",
    "connection_id": "4a37635c-ee2f-466d-8bf9-f7158019a1fc"
}
 */
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ReceiveInvitationResponse {
    pub accept: String,
    pub connection_id: String,
    pub created_at: DateTime<Utc>,
    pub invitation_key: String,
    pub invitation_mode: String,
    pub rfc23_state: String,
    pub routing_state: String,
    pub state: String,
    pub their_role: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L10N {
    pub locale: Locales,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblemReport {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    pub explain: String,
    #[serde(rename = "~i10n")]
    pub i10_n: L10N,                                // NOTE: this is different from RFC 0160 which calls it I10N
    #[serde(rename = "problem-code")]
    pub problem_code: String,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub thid: String,
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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    pub host_type: String,
    pub wallet_type: String,
    pub message: String
}

impl StatusResponse {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
