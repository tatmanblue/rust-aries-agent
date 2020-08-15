use crate::ProtocolMessages:: {
    ErrorResponse,
    GenericResponse,
    BasicMessage::BasicMessage
};

/*
  Agents must implement this trait
 */
pub trait ProtocolTrait {
    // Temporary method to prove out our "DI"
    fn status(&self);
    // TODO: reply?
    fn receive_basic_message(&self, message: BasicMessage) -> Result<GenericResponse, ErrorResponse>;
}
