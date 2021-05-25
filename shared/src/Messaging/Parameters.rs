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