use AriesShared::ProtocolMessages::{
    ErrorResponse,
    GenericResponse,
    BasicMessage
};
use AriesShared::ProtocolTrait::ProtocolTrait;

#[derive(Debug)]
pub struct AgentProtocol {

}

impl ProtocolTrait for AgentProtocol {
    fn status(&self) {
        println!("AgentProtocol implementation");
    }
    fn receive_basic_message(&self, message: BasicMessage) -> Result<GenericResponse, ErrorResponse> {
        println!("Agent received basic message '{}'", message.content);
        let response: GenericResponse = GenericResponse { id: 1 }; 
        Ok(response)
    }
}

