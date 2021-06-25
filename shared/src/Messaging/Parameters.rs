// connections/invitation/url parameters
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct ConnectionInviteUrlParameters {
    pub c_i: String
}

impl Default for ConnectionInviteUrlParameters {
    fn default() -> Self {
        Self {
            c_i: "".to_string(),
        }
    }
}

// connections/create-invitation parameters
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct CreateInvitationParameters {
    pub alias: String,
    pub auto_accept: bool,
    pub multi_use: bool,
    pub public: bool
}

impl Default for CreateInvitationParameters {
    fn default() -> Self {
        Self {
            alias: "".to_string(),
            auto_accept: false,
            multi_use: false,
            public: false
        }
    }
}

// this mirrors CreateInvitationResponse.Invitation type
// /connections/receive-invitation
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct InvitationParameters {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    pub did: String,
    pub image_url: String,
    pub label: String,
    pub recipient_keys: Vec<String>,
    pub routing_keys: Vec<String>,
    pub service_endpoint: String,
}

impl Default for InvitationParameters {
    fn default() -> Self {
        Self {
            id:  "".to_string(),
            recipient_keys: vec![],
            type_field: "".to_string(),
            service_endpoint: "".to_string(),
            image_url: "".to_string(),
            routing_keys: vec![],
            did: "".to_string(),
            label: "".to_string()
        }
    }
}
