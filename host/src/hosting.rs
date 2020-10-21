use AriesAgency::AgencyProtocol::AgencyProtocol;
use AriesAgent::AgentProtocol::AgentProtocol;
use AriesShared::ProtocolMessages::{
    ErrorResponse,
    GenericResponse,
    BasicMessage
};
use AriesShared::ProtocolTrait::ProtocolTrait;


/**
    Think of HostedRoleTypes and `impl ProtocolTrait` as poor man's DI in rust.

    HostedRoleTypes enums becomes a "placeholder" (so to speak) of the concrete implementation of
    ProtocolTrait which handles the specific implementation.   This allows getting around "known size"
    requirements rust has on return types.

    It solves for the O in solid:  open for extension.  add a new enum and you add a new behavior into the system
    without breaking existing implementations

    // ToThink(): the enum and structure may need to be in a different library
    // and will reorganize as pattern emerges

*/
#[derive(Debug, Clone)]
pub enum HostedRoleTypes {
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
impl ProtocolTrait for HostedRoleTypes {
    fn status(&self) {
        match *self {
            HostedRoleTypes::Agent(ref handler) => handler.status(),
            HostedRoleTypes::Agency(ref handler) => handler.status(),
        }
    }

    fn receive_basic_message(&self, message: BasicMessage)  -> Result<GenericResponse, ErrorResponse> {
        match *self {
            HostedRoleTypes::Agent(ref handler) => handler.receive_basic_message(message),
            HostedRoleTypes::Agency(ref handler) => handler.receive_basic_message(message),
        }
    }
}

/*
    This factory finalized our version of DI by returning the correctly initialized "type"
 */
pub fn get_agent_or_agency(role_type: &str) -> HostedRoleTypes {
    // TODO: initialization of agent or agency
    match role_type.to_lowercase().as_str() {
        "agency" => HostedRoleTypes::Agency(AgencyProtocol {}),
        _ => HostedRoleTypes::Agent(AgentProtocol {})
    }
}