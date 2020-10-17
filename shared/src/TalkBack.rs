
/*
    What is TalkBack

    Talkback is an agents ability to send responses to external processes.

    Agents can behave one of two ways: automatically respond to messages, or wait for
    input that tells the agent how to respond.  Talkback allows "3rd party" automation for
    messaging handling by having the agent send the response to another process which then continues
    the makes the necessary steps to continue protocol handling or not.

    toThink(): make a demo of how this automation works
*/
pub enum TalkBackTypes {
    None(NoneTalkBackHandler),
    Http(HttpTalkbackHandler)
}

// TODO: this is work in progress
pub trait TalkBackTrait {
    fn sendMessage(&self, topic: &str, sub_topic: &str, message: &str);   // TBD
}

pub struct TalkBackFactory {}

impl TalkBackFactory {
    pub fn get_talk_back_handler(talk_back_type: &str, talk_back_config: &str) -> TalkBackTypes {
        match talk_back_type.to_lowercase().as_str() {
            "http" => TalkBackTypes::Http(HttpTalkbackHandler { url : talk_back_config.to_string()}),
            _ => TalkBackTypes::None(NoneTalkBackHandler {})
        }
    }
}

// The default is to do nothing.  So this handler is just an empty do nothing implementation
pub struct NoneTalkBackHandler {}

impl TalkBackTrait for NoneTalkBackHandler {
    fn sendMessage(&self, _topic: &str, _sub_topic: &str, _message: &str) {
        debug!("no talk back handler has been defined");
    }
}

// This handler will send out talk back messages via a url
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
pub struct HttpTalkbackHandler {
    pub url: String
}

impl TalkBackTrait for HttpTalkbackHandler {
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