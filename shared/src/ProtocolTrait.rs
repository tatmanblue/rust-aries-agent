use crate::ProtocolMessages:: {
    ErrorResponse,
    GenericResponse,
    BasicMessage
};

/*
    Agents must implement this trait.
    each method is to handle an aries protocol message and provide appropriate reply.
 */
pub trait ProtocolTrait {
    // simply reports the status of the agent
    // TODO: reply???
    fn status(&self);
    // handle an inbound basic message
    fn receive_basic_message(&self, message: BasicMessage) -> Result<GenericResponse, ErrorResponse>;
}
