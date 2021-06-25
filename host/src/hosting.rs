use AriesAgency::AgencyProtocol::AgencyProtocol;
use AriesAgent::AgentProtocol::AgentProtocol;
use AriesShared::{
    Automation::AutomationTypes,
    Messaging::{
        Parameters::*,
        BasicMessage,
        CreateInvitationResponse,
        ErrorResponse,
        GenericResponse,
        ReceiveInvitationResponse,
        StatusResponse
    },
    ProtocolTrait::ProtocolTrait,
    Wallets::WalletTypes
};



/**
    Think of HostedRoleTypes and `impl ProtocolTrait` as poor man's DI in rust.

    HostedRoleTypes enums becomes a "placeholder" (so to speak) of the concrete implementation of
    ProtocolTrait which handles the specific implementation.   This allows getting around "known size"
    requirements rust has on return types.

    It solves for the O in solid:  open for extension.  add a new enum and you add a new behavior into the system
    without breaking existing implementations

    // ToThink(): is a better implementation using generics https://doc.rust-lang.org/book/ch10-00-generics.html

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
    fn status(&self) -> Result<StatusResponse, ErrorResponse>  {
        debug!("HostedRoleTypes.status");
        let mut result = match *self {
            HostedRoleTypes::Agent(ref handler) => handler.status(),
            HostedRoleTypes::Agency(ref handler) => handler.status(),
        };

        // TODO: if success, update host_type and wallet_type
        result
    }

    fn create_invitation_message(&self, params: CreateInvitationParameters) -> Result<CreateInvitationResponse, ErrorResponse> {
        debug!("HostedRoleTypes.receive_create_message");
        match *self {
            HostedRoleTypes::Agent(ref handler) => handler.create_invitation_message(params),
            HostedRoleTypes::Agency(ref handler) => handler.create_invitation_message(params),
        }
    }

    fn list_all_connections(&self) -> Result<GenericResponse, ErrorResponse> {
        debug!("HostedRoleTypes.list_all_connections");
        match *self {
            HostedRoleTypes::Agent(ref handler) => handler.list_all_connections(),
            HostedRoleTypes::Agency(ref handler) => handler.list_all_connections(),
        }
    }

    fn receive_invitation_message(&self, params:InvitationParameters) -> Result<ReceiveInvitationResponse, ErrorResponse> {
        debug!("HostedRoleTypes.receive_start_invitation_message");
        match *self {
            HostedRoleTypes::Agent(ref handler) => handler.receive_invitation_message(params),
            HostedRoleTypes::Agency(ref handler) => handler.receive_invitation_message(params),
        }
    }

    fn receive_basic_message(&self, message: BasicMessage)  -> Result<GenericResponse, ErrorResponse> {
        debug!("HostedRoleTypes.receive_create_message");
        match *self {
            HostedRoleTypes::Agent(ref handler) => handler.receive_basic_message(message),
            HostedRoleTypes::Agency(ref handler) => handler.receive_basic_message(message),
        }
    }
}

/*
    This factory finalized our version of DI by returning the correctly initialized "type"
 */
pub fn get_agent_or_agency(role_type: &str, service_end_point: &str,
                           wallet: WalletTypes, automation: AutomationTypes) -> HostedRoleTypes {
    match role_type.to_lowercase().as_str() {
        "agency" => HostedRoleTypes::Agency(AgencyProtocol {}),
        _ => HostedRoleTypes::Agent(AgentProtocol {
            automation,
            wallet,
            service_end_point: service_end_point.to_string() })
    }
}