use AriesShared::ProtocolMessages::{
    BasicMessage,
    CreateInvitationResponse,
    ErrorResponse,
    GenericResponse,
    StatusResponse
};
use AriesShared::ProtocolTrait::ProtocolTrait;

#[derive(Debug, Clone)]
pub struct AgentProtocol {

}

impl ProtocolTrait for AgentProtocol {
    fn status(&self) -> Result<StatusResponse, ErrorResponse> {
        Ok(StatusResponse {
            message : "Agent reporting status (TODO)".to_string()
        })
    }
    fn receive_create_message(&self) -> Result<CreateInvitationResponse, ErrorResponse> {
        todo!()
    }
    fn receive_basic_message(&self, message: BasicMessage) -> Result<GenericResponse, ErrorResponse> {
        println!("Agent received basic message '{}'", message.content);
        let response: GenericResponse = GenericResponse { id: 1 }; 
        Ok(response)
    }
}

