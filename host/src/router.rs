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

//impl Copy for RouterConfig {}

pub fn new_router(routing_role: &str) -> Router {
    let config: RouterConfig = RouterConfig {
      role: routing_role.to_string()
    };
    let mut app: Server<RouterConfig> = Server::with_state(config);

    let result: Router = Router {
        app
    };

    result
}

pub fn map_all_routes(mut router: Router) {
    router.app.at("/status").get(|_ : Request<RouterConfig>| async {
        // info!("status will come from {:?}", router.role);
        Ok("ok")
    });

    router.app.at("/connections/create-invitation").post(|_ : Request<RouterConfig>| async {
        // todo get body
        Ok("ok")
    });

}

pub fn run_router(router: Router, host: &str) {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(router.app.listen(host.to_string()));
}

impl Router {
    pub fn new(routing_role: &str) -> Router {
        let config: RouterConfig = RouterConfig {
            role:  routing_role.to_string()
        };
        let mut app: Server<RouterConfig> = Server::with_state(config);
        Router {
            app,
        }
    }

    pub fn run(self, host: &str) {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(self.app.listen(host.to_string()));
    }

    pub fn map_all_routes(&mut self) {
        self.app.at("/status").get( |_ : Request<(RouterConfig)>| async {
            // info!("status will come from {:?}", router.role);
            Ok("ok")
        });

        self.app.at("/connections/create-invitation").post(|_ : Request<(RouterConfig)>| async {
            // todo get body
            // self.host_type.create_invitiation();
            Ok("ok")
        });
    }

}
