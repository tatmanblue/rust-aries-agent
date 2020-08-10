#![warn(unused_must_use)]
#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};
use tide;
use tokio;

// ToThink(): the enum and structure may need to be in a different library
// but will reorganize as pattern emerges
enum HostingTypes {
	Agent,
	Agency
}

struct Config {
	host: String,
	host_type: HostingTypes
}

impl Config {
	fn new() -> Self {
		let yaml = load_yaml!("config.yml");
		let options: ArgMatches = App::from_yaml(yaml).get_matches();

		let host = options.value_of("host").unwrap_or("127.0.0.1:8000");
		// TODO: eventually this will come from configuration
		let host_type = HostingTypes::Agent;

		Config {
			host: host.to_string(),
			host_type
		}
	}

	fn print() {

	}
}


fn run_host(host: &str) {
	let mut app = tide::new();
	
	// TODO: let agent add routes
	app.at("/").get(|_| async { Ok("Hello, world!") });
	
	// TODO: the IP port etc come from configuration
	// env, config file, parameter etc....
	
	let mut rt = tokio::runtime::Runtime::new().unwrap();
	rt.block_on(app.listen(host));
}

fn main() {
	let options: Config = Config::new();
	println!("listening on {0}", options.host);
	run_host(&options.host);
}

