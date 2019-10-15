#[macro_use]
extern crate log;
use simple_logger;


use mjolnird::{Config, Mode, cli, server};

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
