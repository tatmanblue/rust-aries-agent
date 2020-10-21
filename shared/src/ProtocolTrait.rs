use crate::ProtocolMessages:: {
    BasicMessage,
    CreateInvitationResponse,
    ErrorResponse,
    GenericResponse,
    StatusResponse
};

/*
    Agents must implement this trait.
    each method is to handle an aries protocol message and provide appropriate reply.
 */
pub trait ProtocolTrait {
    // simply reports the status of the agent
    // TODO: reply???
    fn status(&self) -> Result<StatusResponse, ErrorResponse>;
    fn receive_create_message(&self) -> Result<CreateInvitationResponse, ErrorResponse>;
    // handle an inbound basic message
    fn receive_basic_message(&self, message: BasicMessage) -> Result<GenericResponse, ErrorResponse>;
}
