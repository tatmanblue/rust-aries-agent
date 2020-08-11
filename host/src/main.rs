#![warn(unused_must_use)]

// our dependencies, and keep them alphabetical
#[macro_use]
extern crate clap;

// our mods, and keep them alphabetical
mod hosting;

// dependency use statements, and keep them alphabetical
use clap::{App, ArgMatches};
use tide;
use tokio;

// our use statements, and keep them alphabetical (getting the idea yet?)
use hosting::{HostingFactory, HostingTypes};

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
		let host_type = HostingFactory::get_agent_or_agency();

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

