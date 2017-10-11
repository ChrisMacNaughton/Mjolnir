#[macro_use] extern crate clap;
extern crate mjolnir;

use std::net::SocketAddr;
use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    println!("Welcome to Mjölnir");
    
    let config = get_config();
    println!("About to start with {:?}", config);
}

#[derive(Debug)]
struct Config {
    bind_address: SocketAddr,
    mode: Mode,
}

#[derive(Debug)]
enum Mode {
    Agent(Vec<SocketAddr>),
    Master
}
fn get_config() -> Config {
    let matches = App::new("Mjölnir")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("bind")
                .help("What address:port to bind to")
                .long("bind")
                .short("b")
                .takes_value(true)
        )
        .subcommand(
            SubCommand::with_name("agent")
                .help("THe machine agent that runs on every machine")
                .arg(
                    Arg::with_name("master")
                        .help("IP Address[es] of the master")
                        .long("master")
                        .short("m")
                        .required(true)
                        .takes_value(true)
                        .multiple(true))
        ).subcommand(
            SubCommand::with_name("master")
                .help("The daemon that controls everything")
        ).get_matches();

        let mode = match matches.subcommand() {
            ("master", Some(master_matches)) => {
                Mode::Master
            }
            ("agent", Some(agent_matches)) => {
                // This unwrap is safe because we declare masters as required
                let masters = agent_matches.values_of("master").unwrap().map(|ip|
                    ip.parse().expect(&format!("{} is an invalid address", ip)[..])
                ).collect();
                Mode::Agent(masters)
            }
            (_, _) => unreachable!()
        };
        let address = matches.value_of("bind").unwrap_or_else(|| {
            match mode {
                Mode::Master => "0.0.0.0:11011",
                Mode::Agent(_) => "0.0.0.0:11012"
            }
        }).parse().expect("You provided an invalid bind address");

        Config {
            bind_address: address,
            mode: mode,
        }
}