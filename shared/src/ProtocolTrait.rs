
/*
  Agents must implement this trait
 */
pub trait ProtocolTrait {
    // Temporary method to prove out our "DI"
    fn status(&self);
    // TODO: body?
    fn receive_basic_message(&self);
}
