#![feature(async_await, futures_api, await_macro)]

#[macro_use]
extern crate serde_derive;

use tide::{error::ResultExt, response, App, Context, EndpointResult};
use http::status::StatusCode;

fn main() {
    println!("Rust Aries HOST");
	let mut app = App::new();
	
	// TODO: the IP port etc come from configuration
	// env, config file, parameter etc....
	app.serve("127.0.0.1:8000").unwrap();
}

