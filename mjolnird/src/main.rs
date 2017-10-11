#[macro_use] extern crate clap;
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate mjolnir;

mod config;
mod server;

use config::Config;

fn main() {
    println!("Welcome to Mjölnir");
    
    let config = Config::get_config();
    println!("About to start with {:?}", config);

    server::bind(&config).expect("Couldn't bind to the specified port");
}
