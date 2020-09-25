
pub mod BasicMessage;

// toThink(): should all message structures be in this file
// while keeping impl in specific mod.  Advantage is it 
// would clean up use clauses 
// eg:
// use AriesShared::ProtocolMessages::BasicMessage::BasicMessage becomes
// use AriesShared::ProtocolMessages::BasicMessage

// TODO: define what a GenericResponse means
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericResponse {
    pub id: u32
}

// TODO: define what a ErrorResponse means
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub id: u32
}