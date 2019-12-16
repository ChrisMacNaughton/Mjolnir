#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
// workspace members



mod config;
pub mod server;

pub use config::{Config, Mode, CliMode};

pub fn cli(config: &Config, mode: CliMode) {
    info!("Running {:?}", mode);
    for master in config.masters.iter() {
        if let Some(key) = server::get_master_pubkey(&master) {
            server::reload(&master, &key);
        }
    }
}