
/*
    What is Automation

    Automation is an agents ability to send state responses to external processes.  Please refer
    to different aries RFCs for state data.

    Agents can behave one of two ways: automatically respond to messages, or wait for
    input that tells the agent how to respond.  Automation allows "3rd party" automation for
    messaging handling by having the agent send the response to another process which then continues
    the makes the necessary steps to continue protocol handling or not.

    toThink(): make a demo of how this automation works
*/
#[derive(Debug, Clone)]
pub enum AutomationTypes {
    None(NoAutomationHandler),
    Http(HttpAutomationHandler)
}

// TODO: this is work in progress
pub trait AutomationTrait {
    fn sendMessage(&self, topic: &str, sub_topic: &str, message: &str);   // TBD
}

pub fn get_automation_handler(automation_type: &str, automation_config: &str) -> AutomationTypes {
    match automation_type.to_lowercase().as_str() {
        "http" => AutomationTypes::Http(HttpAutomationHandler { url : automation_config.to_string()}),
        _ => AutomationTypes::None(NoAutomationHandler {})
    }
}

// The default is to do nothing.  So this handler is just an empty do nothing implementation
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct NoAutomationHandler {}

impl AutomationTrait for NoAutomationHandler {
    fn sendMessage(&self, _topic: &str, _sub_topic: &str, _message: &str) {
        debug!("no talk back handler has been defined");
    }
}

// This handler will send out automation messages via a url
//
// url is user defined.  It could be something like http://localhost:5001 or
// http://localhost:5001/talkback
//
// It will also send the message as POST with message sent in the body, expecting message
// to be formatted as json.
//
// The topic and subtopic will become additional routes added to the url
// so it would be something like this:
// POST http://localhost:5001/topic/subtopic
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HttpAutomationHandler {
    pub url: String
}

impl AutomationTrait for HttpAutomationHandler {
    fn sendMessage(&self, topic: &str, sub_topic: &str, message: &str) {
        debug!("http talkBack enabled {:?}/{:?} data {:?}", topic, sub_topic, message);

        let url: String = format!("{}/{}/{}", self.url, topic, sub_topic);

        // TODO: make this post async
        let client = reqwest::blocking::Client::new();
        let _res = client.post(&url)
            .body(message.to_string())
            .send();

        // TODO: report on error as it may help with debugging/support
    }
}