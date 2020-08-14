use AriesAgency::AgencyProtocol::AgencyProtocol;
use AriesAgent::AgentProtocol::AgentProtocol;
use AriesShared::ProtocolTrait::ProtocolTrait;


/**
    Think of HostingTypes and `impl ProtocolTrait` as poor man's DI in rust.

    HostingTypes enums becomes a "placeholder" (so to speak) of the concrete implementation of
    ProtocolTrait which handles the specific implementation.   This allows getting around "known size"
    requirements rust has on return types.

    It solves for the O in solid:  open for extension.  add a new enum and you add a new behavior into the system
    without breaking existing implementations

    // ToThink(): the enum and structure may need to be in a different library
    // but will reorganize as pattern emerges

*/
#[derive(Debug)]
pub enum HostingTypes {
    Agent(AgentProtocol),
    Agency(AgencyProtocol)
}

/**
    Think of this ProtocolTrait implementation as something along the lines of an abstract class pointer
    or a v-table pointer in an OO language.
    
    The factory returns one of the enums along  with the associated implementation type allocated within the enum itself.
    Since consumers have the enum, the ProtocolTrait implementation on the enum allows the consumer to work with the
    enum as a "class pointer" and not have to know about the actual implementation details; thereby making the
    consumer decoupled from the implementation.  This solves for the "i" in SOLID

    The concrete implementation are in their respective libraries.
*/
impl ProtocolTrait for HostingTypes {
    fn status(&self) {
        match *self {
            HostingTypes::Agent(ref handler) => handler.status(),
            HostingTypes::Agency(ref handler) => handler.status(),
        }
    }

    fn receive_basic_message(&self) {
        match *self {
            HostingTypes::Agent(ref handler) => handler.receive_basic_message(),
            HostingTypes::Agency(ref handler) => handler.receive_basic_message(),
        }
    }
}

pub struct HostingFactory {}

/*
    This factory finalized our version of DI by returning the correctly initialized "type"
 */
impl HostingFactory {
    pub fn get_agent_or_agency(hosting_type: &str) -> HostingTypes {
        // TODO: initialization of
        match hosting_type.to_lowercase().as_str() {
            "agency" => HostingTypes::Agency(AgencyProtocol {}),
            _ => HostingTypes::Agent(AgentProtocol {})
        }
    }
}
