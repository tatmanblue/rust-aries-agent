#[macro_use]
extern crate serde_derive;

use tide;
use tokio;

fn run_host() {
	let mut app = tide::new();
	
	// TODO: let agent add routes
	app.at("/").get(|_| async { Ok("Hello, world!") });
	
	// TODO: the IP port etc come from configuration
	// env, config file, parameter etc....
	
	let mut rt = tokio::runtime::Runtime::new().unwrap();
	rt.block_on(app.listen("127.0.0.1:8000"));
}

fn main() {
    println!("Rust Aries initializing");	
	run_host();
}

