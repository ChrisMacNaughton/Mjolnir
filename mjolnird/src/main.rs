extern crate base64;
#[macro_use]
extern crate clap;
extern crate futures;
extern crate hostname;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate simple_logger;
extern crate tokio_core;
extern crate toml;
extern crate uuid;
extern crate xdg;
extern crate yaml_rust;
extern crate zmq;
// workspace members
extern crate mjolnir;
extern crate mjolnir_api;

mod config;
mod server;

use config::{Config, Mode, CliMode};

fn main() {
    println!("Welcome to Mjölnir");
    trace!("Loading config");
    let config = Config::get_config();
    let _ = simple_logger::init_with_level(config.log_level);
    trace!("Loaded config: {:?}", config);
    match config.mode {
        Mode::Cli(mode) => {
            cli(&config, mode);
            return;
        },
        _ => {}
    }
    server::bind(config).expect("Couldn't bind to the specified port");
}

fn cli(config: &Config, mode: CliMode) {
    info!("Running {:?}", mode);
    for master in config.masters.iter() {
        if let Some(key) = server::get_master_pubkey(&master) {
            server::reload(&master, &key);
        }
    }
}