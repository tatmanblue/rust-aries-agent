//
//
// responsible for setting up url route to hosting mapping
//
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
        self.app.at("/status").get( |request : Request<RouterConfig>| async move {
            let config: &RouterConfig = request.state();
            config.mediator.status();
            Ok("ok")
        });

        self.app.at("/connections/create-invitation").post(|request : Request<RouterConfig>| async move {
            let config: &RouterConfig = request.state();
            info!("mediator type {:?} will handle create-invitation", config.role);
            Ok("ok")
        });
    }

}
