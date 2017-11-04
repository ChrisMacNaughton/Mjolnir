#[macro_use] extern crate clap;
extern crate hostname;
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate protobuf;
extern crate xdg;
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
