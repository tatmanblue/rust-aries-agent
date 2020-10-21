//
//
// responsible for setting up url route to hosting mapping
//
use http::Error;
use tide::{Request, Response, Result, Server};

use AriesShared::ProtocolTrait::ProtocolTrait;
use super::hosting::{HostedRoleTypes, get_agent_or_agency};

pub struct Router {
    app: Server<RouterConfig>
}

#[derive(Debug, Clone)]
pub struct RouterConfig {
    role: String,
    mediator: HostedRoleTypes
}

impl Router {
    pub fn new(routing_role: &str) -> Router {
        let mediator: HostedRoleTypes = get_agent_or_agency(routing_role);
        let config: RouterConfig = RouterConfig {
            role:  routing_role.to_string(),
            mediator
        };
        let app: Server<RouterConfig> = Server::with_state(config);
        Router {
            app,
        }
    }

    pub fn run(self, host: &str) {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        match rt.block_on(self.app.listen(host.to_string())) {
            Ok(_) => { },
            Err(e) => {
                error!("webhosting error {:?}", e)
            }
        }
    }

    pub fn map_all_routes(&mut self) {
        self.app.at("/status").get(Router::get_status);
        self.app.at("/connections/create-invitation").post(Router::create_invitation);
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

    async fn create_invitation(request : Request<RouterConfig>) -> Result<Response> {
        let config: &RouterConfig = request.state();
        let mut response = Response::builder(400);
        match config.mediator.receive_create_message() {
            Ok(invite) => {
                response = Response::builder(200).content_type("application/json").body(invite.to_json());
            },
            Err(e) => {
                warn!("status error {:?}", e);
            }
        }
        Ok(response.build())
    }
}
