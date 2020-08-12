use AriesShared::ProtocolTrait::ProtocolTrait;

#[derive(Debug)]
pub struct AgencyProtocol {

}

impl ProtocolTrait for AgencyProtocol {
    fn status(&self) {
        println!("AgencyProtocol implementation");
    }
}
