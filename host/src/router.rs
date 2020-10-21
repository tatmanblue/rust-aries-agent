//
//
// responsible for setting up url route to hosting mapping
//
use tide::{Request, Response, Result, Server};

use AriesShared::ProtocolTrait::ProtocolTrait;
use super::hosting::{HostingRoleTypeFactory, HostedRoleTypes};

pub struct Router {
    app: Server<()>,
    role: String
}

pub fn new_router(role: &'static str) -> Router {
    let mut app: Server<()> = tide::new();

    /*
    app.at("/status").get(|_ : Request<()>| async {
        info!("status will come from {:?}", role.to_string());
        Ok("ok")
    });

    app.at("/connections/create-invitation").post(|_ : Request<()>| async {
        // todo get body
        Ok("ok")
    }); */

    let result: Router = Router {
        app,
        role: role.to_string()
    };

    result
}

pub fn map_all_routes(mut router: Router) {
    router.app.at("/status").get(|_ : Request<()>| async {
        // info!("status will come from {:?}", router.role);
        Ok("ok")
    });

    router.app.at("/connections/create-invitation").post(|_ : Request<()>| async {
        // todo get body
        Ok("ok")
    });

}

pub fn run_router(router: Router, host: &str) {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(router.app.listen(host.to_string()));
}

impl Router {
    pub fn new(role: &str) -> Router {
        let app: Server<()> = tide::new();
        Router {
            app,
            role: role.to_string()
        }
    }

    pub fn run(self, host: &str) {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(self.app.listen(host.to_string()));
    }

    pub fn map_all_routes(&mut self) {
        self.app.at("/status").get( |_ : Request<()>| async {
            // info!("status will come from {:?}", router.role);
            Ok("ok")
        });

        self.app.at("/connections/create-invitation").post(|_ : Request<()>| async {
            // todo get body
            // self.host_type.create_invitiation();
            Ok("ok")
        });
    }

}
