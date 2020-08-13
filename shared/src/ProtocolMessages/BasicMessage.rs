use std::io::Error;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicMessage {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "~l10n")]
    pub l10n: L10N,
    #[serde(rename = "sent_time")]
    pub sent_time: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L10N {
    pub locale: String,
}

impl BasicMessage {
    pub fn new(id: String) -> BasicMessage {
        BasicMessage {
            id,
            type_field: "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/basicmessage/1.0/message".to_string(),
            l10n: L10N {
                locale: "".to_string()
            },
            sent_time: "".to_string(),
            content: "".to_string(),
        }
    }
    
    pub fn from_json(json: &str) -> Result<BasicMessage, Error> {
        let message: BasicMessage = serde_json::from_str(json)?;
        Ok(message)
    }
}