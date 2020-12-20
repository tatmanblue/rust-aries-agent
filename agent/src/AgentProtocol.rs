use base64_url::{encode};

use AriesShared::{
    Crypto::{ Did },
    Messaging::{
        Parameters::{
            CreateInvitationParameters
        },
        BasicMessage,
        CreateInvitationResponse,
        ErrorResponse,
        GenericResponse,
        StatusResponse
    },
    ProtocolTrait::ProtocolTrait,
    Wallets::WalletTypes
};

#[derive(Debug, Clone)]
pub struct AgentProtocol {
    pub wallet: WalletTypes,
    pub service_end_point: String           // URL, message queue URI etc....
}

/*
    toThink():
    unfortunately this implementation uses http specific data types.  This needs to be better abstracted
*/
impl ProtocolTrait for AgentProtocol {
    fn status(&self) -> Result<StatusResponse, ErrorResponse> {
        Ok(StatusResponse {
            message : "Agent reporting status (TODO)".to_string()
        })
    }

    fn receive_create_invitation_message(&self, params: CreateInvitationParameters) -> Result<CreateInvitationResponse, ErrorResponse> {
        let mut response: CreateInvitationResponse = CreateInvitationResponse::new();
        let did : Did = Did::new(None);

        // TODO: need some kind of resource that provides URL formatting.  cannot assume
        // it is http since down the road that could be message queue etc...

        // TODO: not using at this time -> update response.invitation.routing_keys
        // TODO: not using at this time -> update invitation.invitation.image_url
        response.invitation.service_endpoint = format!("http://{}", self.service_end_point.to_string());
        response.invitation.did = format!("did:sov:{}", did.did);
        response.invitation.label = params.alias.to_string();
        response.invitation.recipient_keys.push(did.did.to_string());

        // TODO: get alias from params if they exist
        let encoded_invitation = encode(&response.invitation.to_json());
        response.invitation_url = format!("http://{}/connections/url?c_i={}", self.service_end_point.to_string(), encoded_invitation);

        // TODO: save invitation so that we can reference it when returned by interested party

        Ok(response)
    }

    fn list_all_connections(&self) -> Result<GenericResponse, ErrorResponse> {
        todo!()
    }

    fn receive_basic_message(&self, message: BasicMessage) -> Result<GenericResponse, ErrorResponse> {
        println!("Agent received basic message '{}'", message.content);
        let response: GenericResponse = GenericResponse { id: 1 }; 
        Ok(response)
    }
}

