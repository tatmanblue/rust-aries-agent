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

#[cfg(test)]
mod basic_message_test {
    use super::*;

    #[test]
    fn test_json_converts_ok() {
        const JSON: &str = r#"{
            "@id": "123456780",
            "@type": "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/basicmessage/1.0/message",
            "~l10n": { "locale": "en" },
            "sent_time": "2019-01-15 18:42:01Z",
            "content": "Your hovercraft is full of eels."
            }"#;
        
        let message: BasicMessage = BasicMessage::from_json(JSON).unwrap();
        assert_eq!(message.id, "123456780");
        assert_eq!(message.content, "Your hovercraft is full of eels.");
    }
}