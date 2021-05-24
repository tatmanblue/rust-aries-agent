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
use env_logger::{Builder};

// our mods, and keep them alphabetical
mod hosting;
mod router;

// dependency use statements, and keep them alphabetical
use clap::{App, ArgMatches};
use std::fs;

// our use statements, and keep them alphabetical (getting the idea yet?)
use AriesShared::{
	Automation::{
		AutomationTypes,
		get_automation_handler
	},
	Wallets::{
		WalletTypes,
		get_wallet_handler
	}
};
use hosting::{HostedRoleTypes, get_agent_or_agency};
use router::{Router};
use AriesShared::Wallets::WalletTrait;

pub struct Config {
	host: String,
	role: String,			// host_type
	wallet_type: String,
	wallet_config: String,
	wallet_config_file: String,
	automation_type: String,
	automation_config: String
}

impl Config {
	fn new() -> Self {
		let yaml = load_yaml!("config.yml");
		let options: ArgMatches = App::from_yaml(yaml).get_matches();

		let host: &str = options.value_of("host").unwrap_or("127.0.0.1:8000");
		let wallet_config: &str = options.value_of("walletConfig").unwrap_or("");
		let talk_back_config: &str = options.value_of("automationConfig").unwrap_or("");
		let role: &str = options.value_of("role").unwrap_or("Agent");
		let wallet_type: &str = options.value_of("walletType").unwrap_or("Basic");
		let wallet_config_file: &str = options.value_of("walletConfigFile").unwrap_or("");
		let talk_back_type: &str = options.value_of("automation").unwrap_or("none");

		Config {
			host: host.to_string(),
			role: role.to_string(),
			wallet_type: wallet_type.to_string(),
			wallet_config: wallet_config.to_string(),
			wallet_config_file: wallet_config_file.to_string(),
			automation_type: talk_back_type.to_string(),
			automation_config: talk_back_config.to_string()
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
	let automation: AutomationTypes = get_automation_handler(&CONFIG.automation_type, &CONFIG.automation_config);

	let mut wallet_config_data: String = CONFIG.wallet_config.to_string();
	if 0 < CONFIG.wallet_config_file.chars().count() {
		wallet_config_data = fs::read_to_string(&CONFIG.wallet_config_file).unwrap();
	}

	let mut wallet: WalletTypes = get_wallet_handler(&CONFIG.wallet_type, &wallet_config_data);
	wallet.open();

	let mediator: HostedRoleTypes = get_agent_or_agency(&CONFIG.role, &CONFIG.host, wallet, automation);
	let mut router: Router = Router::new(&CONFIG.role, mediator);
	router.map_all_routes();
	router.run(&CONFIG.host);

}

fn main() {
	let mut builder = Builder::from_default_env();
	builder.init();

	CONFIG.print();
	run_host();
}

