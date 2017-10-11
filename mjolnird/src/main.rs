#[macro_use] extern crate clap;
extern crate mjolnir;

mod config;

use config::{Config, Mode};

fn main() {
    println!("Welcome to Mjölnir");

    let config = Config::get_config();
    println!("About to start with {:?}", config);
}