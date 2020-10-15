use std::io::Error;
use chrono::Utc;

use crate::Locale::Locales;

use super::{BasicMessage, L10N};


impl BasicMessage {
    pub fn new(id: &str, content: &str) -> BasicMessage {
        BasicMessage {
            id: id.to_string(),
            type_field: "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/basicmessage/1.0/message".to_string(),
            l10n: L10N {
                locale: Locales::enUS
            },
            sent_time: Utc::now(),
            content: content.to_string(),
        }
    }
    
    pub fn from_json(json: &str) -> Result<BasicMessage, Error> {
        let message: BasicMessage = serde_json::from_str(json)?;
        Ok(message)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod Test {
    use super::*;

    #[test]
    fn json_deserializes_ok() {
        const JSON: &str = r#"{
            "@id": "123456780",
            "@type": "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/basicmessage/1.0/message",
            "~l10n": { "locale": "enUS" },
            "sent_time": "2019-01-15 18:42:01Z",
            "content": "Your hovercraft is full of eels."
            }"#;
        
        let message: BasicMessage = BasicMessage::from_json(JSON).unwrap();
        assert_eq!(message.id, "123456780");
        assert_eq!(message.content, "Your hovercraft is full of eels.");
    }

    #[test]
    #[should_panic]
    fn json_deserializes_bad_locale_panics() {
        const JSON: &str = r#"{
            "@id": "123456780",
            "@type": "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/basicmessage/1.0/message",
            "~l10n": { "locale": "rum" },
            "sent_time": "2019-01-15 18:42:01Z",
            "content": "Your hovercraft is full of eels."
            }"#;

        let _ = BasicMessage::from_json(JSON).unwrap();
    }

    #[test]
    fn basic_message_serializes_ok() {
        let message: BasicMessage = BasicMessage::new("Id99", "This is the real message");

        let json = message.to_json();
        let secondMessage: BasicMessage = BasicMessage::from_json(&json).unwrap();
        assert_eq!(message.id, secondMessage.id);
        assert_eq!(message.content, secondMessage.content);
    }
}
