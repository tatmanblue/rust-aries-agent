#![warn(unused_must_use)]
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use tide;
use tokio;

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

    println!("Rust Aries initializing");
	// The YAML file is found relative to the current file, similar to how modules are found
	let yaml = load_yaml!("config.yml");
	let options = App::from_yaml(yaml).get_matches();
	let host = options.value_of("host").unwrap_or("127.0.0.1:8000");

	run_host(host);
}

