use AriesShared::ProtocolMessages::{
    ErrorResponse,
    GenericResponse,
    BasicMessage
};
use AriesShared::ProtocolTrait::ProtocolTrait;

#[derive(Debug)]
pub struct AgencyProtocol {

}

impl ProtocolTrait for AgencyProtocol {
    fn status(&self) {
        println!("AgencyProtocol implementation");
    }
    fn receive_basic_message(&self, message: BasicMessage) -> Result<GenericResponse, ErrorResponse> {
        todo!()
    }
}
