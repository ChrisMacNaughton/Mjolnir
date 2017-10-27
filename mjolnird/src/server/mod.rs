use hyper::Error;
use config::{Config, Mode};

mod master;
mod agent;

// const PHRASE: &'static str = "Hello, World!";

pub fn bind(config: &Config) -> Result<(), Error> {
    match config.mode {
        Mode::Agent(ref masters) => agent::Agent::bind(config.bind_address.clone(), masters.clone()),
        Mode::Master => master::Master::bind(config.bind_address.clone()),
    }
}