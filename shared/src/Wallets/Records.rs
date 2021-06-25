use uuid::Uuid;

// -----------------------------------------------------------------------------------------------
// This file is likely to get long.  Keep all structures organized alphabetically.
//
// note about field ordering:
// with the exception of 'id' and 'type' fields, all fields should be arranged alphabetically
// -----------------------------------------------------------------------------------------------


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionRecord {
    pub id: String,
    pub did: String,
    pub image_url: String,
    pub label: String,
    pub recipient_keys: Vec<String>,
    pub routing_keys: Vec<String>,
    pub service_endpoint: String,
    pub state: ConnectionState,
}

impl ConnectionRecord {
    pub fn new() -> ConnectionRecord {
        ConnectionRecord {
            id: Uuid::new_v4().to_string(),
            did: "".to_string(),
            image_url: "".to_string(),
            label: "".to_string(),
            recipient_keys: vec![],
            routing_keys: vec![],
            service_endpoint: "".to_string(),
            state: ConnectionState::None
        }
    }
}

// connect state mirrors rfc https://github.com/hyperledger/aries-rfcs/tree/master/features/0160-connection-protocol#states
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionState {
    None,
    Invited,
    Requested,
    Responded,
    Complete,
    Error                       // RFC doesn't explicitly have this state, its for our use
}

impl Default for ConnectionState {
    fn default() -> Self { ConnectionState::None }
}