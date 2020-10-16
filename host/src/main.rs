#![allow(unused_must_use)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// our dependencies, and keep them alphabetical
extern crate aries_agency as AriesAgency;
extern crate aries_agent as AriesAgent;
extern crate aries_shared as AriesShared;

// our imported mods
use env_logger::Env;


// our mods, and keep them alphabetical
mod hosting;

// dependency use statements, and keep them alphabetical
use AriesShared::ProtocolTrait::ProtocolTrait;
use clap::{App, ArgMatches};
use tide::*;
use tokio;

// our use statements, and keep them alphabetical (getting the idea yet?)
use hosting::{HostingFactory, HostingTypes};
use AriesShared::Wallets::{WalletTypeFactory, WalletTypes};
use AriesShared::ProtocolMessages::{
	ErrorResponse,
	GenericResponse,
	BasicMessage
};

struct Config {
	host: String,
	host_type: HostingTypes,
	wallet_type: WalletTypes
}

impl Config {
	fn new() -> Self {
		let yaml = load_yaml!("config.yml");
		let options: ArgMatches = App::from_yaml(yaml).get_matches();

		let host: &str = options.value_of("host").unwrap_or("127.0.0.1:8000");
		let host_type: HostingTypes = HostingFactory::get_agent_or_agency(options.value_of("role").unwrap_or("Agent"));
		let wallet_config: &str = options.value_of("walletConfig").unwrap_or("");
		// toThink(): this function breaks the rule of new for config type as we are now actually opening
		// a wallet
		let wallet_type: WalletTypes = WalletTypeFactory::get_wallet_handler(options.value_of("walletType").unwrap_or("Basic"), wallet_config);

		Config {
			host: host.to_string(),
			host_type,
			wallet_type
		}
	}

	fn print(&self) {
		info!("");
		info!("----------------------------------------");
		info!("");
		info!("    Port  {}", self.host);
		info!("    As    {:?}", self.host_type);
		info!("");
		info!("    Wallet {:?}", self.wallet_type);
		info!("----------------------------------------");
		info!("");
	}
}

lazy_static! {
	static ref CONFIG: Config = Config::new();
}

fn run_host() {

	let mut app = tide::new();

	// TODO: temporary impl just to have endpoint working
	app.at("/").get(|_| async move {
		CONFIG.host_type.status();
		Ok("ok")
	});
	
	// TODO: make all routing handled in a different place,
	//       prob something more data driven.  Below is just
	//       temporary while proving out a few things
	// FUTURE PRs will not be allowed to add new routes here
	// TODO: need to confirm api end point
	app.at("/basicmessage").post(|mut req: tide::Request<()>| async move {
		let message: BasicMessage = req.body_json().await.unwrap();
		match CONFIG.host_type.receive_basic_message(message) {
			Ok(_success) => {
				let res = Response::new(200);
				Ok(res)
			},
			_ => {
				let res = Response::new(500);
				Ok(res)
			}
		}
	});

	let mut rt = tokio::runtime::Runtime::new().unwrap();
	rt.block_on(app.listen(CONFIG.host.to_string()));
}

fn main() {
	env_logger::from_env(Env::default().default_filter_or("debug")).init();

	CONFIG.print();
	run_host();
}

