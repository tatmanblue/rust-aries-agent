//
//
// responsible for setting up url route to hosting mapping
//
use tide::{Request, Response, Result, Server};

use AriesShared::{
    Messaging::{
        Parameters::*
    },
    ProtocolTrait::ProtocolTrait
};
use super::{
    hosting::{HostedRoleTypes}
};

pub struct Router {
    app: Server<RouterConfig>
}

#[derive(Debug, Clone)]
pub struct RouterConfig {
    role: String,
    mediator: HostedRoleTypes
}

impl Router {
    pub fn new(role: &str, mediator: HostedRoleTypes) -> Router {
        let config: RouterConfig = RouterConfig {
            role:  role.to_string(),
            mediator
        };
        let app: Server<RouterConfig> = Server::with_state(config);
        Router {
            app,
        }
    }

    pub fn run(self, host: &str) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        match rt.block_on(self.app.listen(host.to_string())) {
            Ok(_) => { },
            Err(e) => {
                error!("Routing error {:?}", e)
            }
        }
    }

    pub fn map_all_routes(&mut self) {
        self.app.at("/status").get(Router::get_status);
        self.app.at("/connections/").get(Router::list_all_connections);
        self.app.at("/connections/create-invitation").post(Router::create_invitation);
        self.app.at("/connections/receive-invitation").post(Router::receive_invitation);
    }

    // TODO:  all of these handlers are likely to get long, for sake of small readable files
    // toThink() consider breaking these out into separate files
    async fn get_status(request : Request<RouterConfig>) -> Result<Response> {
        let config: &RouterConfig = request.state();
        let mut response = Response::builder(400);
        match config.mediator.status() {
            Ok(status) => {
                response = Response::builder(200).content_type("application/json").body(status.to_json());
            },
            Err(e) => {
                warn!("status error {:?}", e);
            }
        }

        Ok(response.build())
    }

    async fn list_all_connections(_request : Request<RouterConfig>) -> Result<Response> {
        let response = Response::builder(400);              // will change to mut once we implement body
        warn!("list_all_connections not implemented");
        Ok(response.build())
    }

    // generates an invitation request which another agent will process with their
    // "receive_invitation" end point.
    //
    async fn create_invitation(request : Request<RouterConfig>) -> Result<Response> {
        let config: &RouterConfig = request.state();
        let params: CreateInvitationParameters = request.query()?;
        let mut response = Response::builder(400);
        match config.mediator.create_invitation_message(params) {
            Ok(invite) => {
                response = Response::builder(200).content_type("application/json").body(invite.to_json());
            },
            Err(e) => {
                warn!("create_invitation error {:?}", e);
            }
        }
        Ok(response.build())
    }

    // another agent generated an invitation, which this agent now processes
    async fn receive_invitation(request: Request<RouterConfig>) -> Result<Response> {
        let config: &RouterConfig = request.state();
        let parms: InvitationParameters = request.query()?;
        let mut response = Response::builder(400);
        match config.mediator.receive_invitation_message(parms) {
            Ok(receive_invite) => {
                response = Response::builder(200).content_type("application/json").body(receive_invite.to_json());
            },
            Err(e) => {
                warn!("receive_invitation error {:?}", e);
            }
        }

        Ok(response.build())
    }
}
