//
//
// responsible for setting up url route to hosting mapping
//
use tide::{Request, Response, Result, Server};

use AriesShared::ProtocolTrait::ProtocolTrait;
use super::hosting::{HostingRoleTypeFactory, HostedRoleTypes};

pub struct Router {
    app: Server<RouterConfig>
}

#[derive(Debug, Clone)]
pub struct RouterConfig {
    role: String
}

impl Router {
    pub fn new(routing_role: &str) -> Router {
        let config: RouterConfig = RouterConfig {
            role:  routing_role.to_string()
        };
        let app: Server<RouterConfig> = Server::with_state(config);
        Router {
            app,
        }
    }

    pub fn run(self, host: &str) {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(self.app.listen(host.to_string()));
    }

    pub fn map_all_routes(&mut self) {
        self.app.at("/status").get( |request : Request<RouterConfig>| async move {
            let state: &RouterConfig = request.state();
            info!("status will come from {:?}", state.role);
            Ok("ok")
        });

        self.app.at("/connections/create-invitation").post(|request : Request<RouterConfig>| async move {
            let state: &RouterConfig = request.state();
            info!("status will come from {:?}", state.role);
            Ok("ok")
        });
    }

}
