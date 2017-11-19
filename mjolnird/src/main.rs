#[macro_use]
extern crate clap;
extern crate futures;
extern crate hostname;
extern crate hyper;
extern crate protobuf;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate toml;
extern crate xdg;
extern crate yaml_rust;
extern crate zmq;
// workspace members
extern crate mjolnir;
extern crate mjolnir_api;

mod config;
mod server;

use config::Config;

fn main() {
    println!("Welcome to Mjölnir");

    let config = Config::get_config();
    println!("About to start with {:?}", config);

    server::bind(config).expect("Couldn't bind to the specified port");
}
