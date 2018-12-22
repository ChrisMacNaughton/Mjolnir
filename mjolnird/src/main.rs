
#[macro_use]
extern crate clap;
use futures;

use hyper;
#[macro_use]
extern crate log;

use reqwest;

#[macro_use]
extern crate serde_derive;
use simple_logger;

use toml;

use xdg;

use zmq;
// workspace members



mod config;
mod server;

use crate::config::{Config, Mode, CliMode};

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
        }
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
