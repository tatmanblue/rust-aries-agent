
// -----------------------------------------------------------------------------------------------
// This file is likely to get long.  Keep all structures organized alphabetically.
//
// note about field ordering:
// with the exception of 'id' and 'type' fields, all fields should be arranged alphabetically
// -----------------------------------------------------------------------------------------------


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvitationRecord {
    pub id: String,
    pub did: String,
    pub image_url: String,
    pub label: String,
    pub recipient_keys: Vec<String>,
    pub routing_keys: Vec<String>,
    pub service_endpoint: String,
}