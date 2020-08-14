use AriesShared::ProtocolTrait::ProtocolTrait;

#[derive(Debug)]
pub struct AgentProtocol {

}

impl ProtocolTrait for AgentProtocol {
    fn status(&self) {
        println!("AgentProtocol implementation");
    }
    fn receive_basic_message(&self) { todo!() }
}

