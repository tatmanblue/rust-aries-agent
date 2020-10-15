#![allow(non_snake_case)]
use chrono::DateTime;
use chrono::Utc;
use crate::Locale::Locales;

mod BasicMessageImpl;

// -----------------------------------------------------------------------------------------------
// This file is likely to get long.  Keep all structures organized alphabetically.  It contains
// of the message structures for all aries protocols implemented
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

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L10N {
    pub locale: Locales,
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

// toThink(): this is not a protocol message but a message
// between rust libraries (shared->host etc).  Prob lives 
// elsewhere
// TODO: define what a ErrorResponse means
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub id: u32
}