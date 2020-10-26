use crate::Messaging:: {
    Parameters::{
        CreateInvitationParameters
    },
    BasicMessage,
    CreateInvitationResponse,
    ErrorResponse,
    GenericResponse,
    StatusResponse
};

/*
    Agents must implement this trait.
    each method is to handle an aries protocol message and provide appropriate reply.

    TODO:
    toThink() what if we broke this trait out into multiple traits based on RFC or some other
    logical separation.
    1. This would allow for smaller implementations and less "not implemented"
       empty methods.
    2. It would allow for agents to more selectively pick and chose which protocols they handle
       (when using the default agent built in this project isn't sufficient)
    3. It would allow for easier code separation but may add complexity with sharing common functionality
 */
pub trait ProtocolTrait {
    // Status is our messaging. Allow for agents to query state
    fn status(&self) -> Result<StatusResponse, ErrorResponse>;

    // RFC '0160: Connection Protocol'
    fn receive_create_invitation_message(&self, params: CreateInvitationParameters) -> Result<CreateInvitationResponse, ErrorResponse>;
    fn list_all_connections(&self) -> Result<GenericResponse, ErrorResponse>;

    // RFC '0095-basic-message'
    fn receive_basic_message(&self, message: BasicMessage) -> Result<GenericResponse, ErrorResponse>;
}
