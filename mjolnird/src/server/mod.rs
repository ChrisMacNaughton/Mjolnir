use hyper::Error;
use config::{Config, Mode};

mod master;
mod agent;

// const PHRASE: &'static str = "Hello, World!";

pub fn bind(config: Config) -> Result<(), Error> {
    match config.mode {
        Mode::Agent(masters) => agent::Agent::bind(config.bind_address, masters),
        Mode::Master => master::Master::bind(config),
    }
}