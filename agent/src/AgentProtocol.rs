use AriesShared::ProtocolMessages::{
    Parameters::{
        CreateInvitationParameters
    },
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
    fn receive_create_message(&self, params: CreateInvitationParameters) -> Result<CreateInvitationResponse, ErrorResponse> {
        let mut invitation: CreateInvitationResponse = CreateInvitationResponse::new();
        // TODO: need some kind of resource that provides URL formatting.  cannot assume
        // it is http since down the road that could be message queue etc...

        // TODO: do we need to generate an alias if none is provided?
        // TODO: update invitation.invitation_url
        // TODO: update invitation.invitation.recipient_keys
        // TODO: update invitation.invitation.service_endpoint
        // TODO: update invitation.invitation.routing_keys
        // TODO: update invitation.invitation.did
        invitation.invitation.label = params.alias.to_string();

        // toThink(): future versions
        // update invitation.invitation.image_url

        Ok(invitation)
    }
    fn receive_basic_message(&self, message: BasicMessage) -> Result<GenericResponse, ErrorResponse> {
        println!("Agent received basic message '{}'", message.content);
        let response: GenericResponse = GenericResponse { id: 1 }; 
        Ok(response)
    }
}

