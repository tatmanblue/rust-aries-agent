// #![allow(unused_must_use)]			// TODO: turn these off
// #![allow(unused_imports)]			// TODO: turn these off

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
mod router;

// dependency use statements, and keep them alphabetical
use clap::{App, ArgMatches};

// our use statements, and keep them alphabetical (getting the idea yet?)
use router::{Router, map_all_routes, run_router};

struct Config {
	host: String,
	role: String,			// host_type
	wallet_type: String,
	wallet_config: String,
	talk_back_type: String,
	talk_back_config: String
}

impl Config {
	fn new() -> Self {
		let yaml = load_yaml!("config.yml");
		let options: ArgMatches = App::from_yaml(yaml).get_matches();

		let host: &str = options.value_of("host").unwrap_or("127.0.0.1:8000");
		let wallet_config: &str = options.value_of("walletConfig").unwrap_or("");
		let talk_back_config: &str = options.value_of("talkBackConfig").unwrap_or("");
		let role: &str = options.value_of("role").unwrap_or("Agent");
		let wallet_type: &str = options.value_of("walletType").unwrap_or("Basic");
		let talk_back_type: &str = options.value_of("talkBack").unwrap_or("none");

		Config {
			host: host.to_string(),
			role: role.to_string(),
			wallet_type: wallet_type.to_string(),
			wallet_config: wallet_config.to_string(),
			talk_back_type: talk_back_type.to_string(),
			talk_back_config: talk_back_config.to_string()
		}
	}

	fn print(&self) {
		info!("");
		info!("----------------------------------------");
		info!("");
		info!("    Port  {}", self.host);
		info!("    As    {:?}", self.role);
		info!("");
		info!("    Wallet {:?}", self.wallet_type);
		info!("----------------------------------------");
		info!("");
	}
}

lazy_static! {
	static ref CONFIG: Config = Config::new();
}

// TODO: someday this method should be data drive to load a "router" based on configuration
// (if we decide one host handles multiple input types)
fn run_host() {
	let mut router: Router = Router::new(&CONFIG.role);
	map_all_routes(router);
	// run_router(router, &CONFIG.host);
}

fn main() {
	env_logger::from_env(Env::default().default_filter_or("debug")).init();

	CONFIG.print();
	run_host();
}

