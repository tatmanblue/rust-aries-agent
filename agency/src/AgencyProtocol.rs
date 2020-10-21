use AriesShared::ProtocolMessages::{
    BasicMessage,
    CreateInvitationResponse,
    ErrorResponse,
    GenericResponse,
    StatusResponse
};
use AriesShared::ProtocolTrait::ProtocolTrait;

#[derive(Debug, Clone)]
pub struct AgencyProtocol {

}

impl ProtocolTrait for AgencyProtocol {
    fn status(&self) -> Result<StatusResponse, ErrorResponse> {
        todo!()
    }
    fn receive_basic_message(&self, _: BasicMessage) -> Result<GenericResponse, ErrorResponse> {
        todo!()
    }
    fn receive_create_message(&self) -> Result<CreateInvitationResponse, ErrorResponse> {
        todo!()
    }
}
