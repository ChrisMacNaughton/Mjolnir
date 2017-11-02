use std::net::SocketAddr;
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches, SubCommand};
use xdg;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_a_default_master_config() {
        let args = Config::matches().get_matches_from(vec!["mjolnird", "master"]);
        let config = Config::from_args(args);
        assert_eq!(config.mode, Mode::Master);
        assert_eq!(config.bind_address, "0.0.0.0:11011".parse().unwrap());
    }

    #[test]
    fn it_builds_a_master_config() {
        let args = Config::matches()
            .get_matches_from(vec!["mjolnird", "--bind=192.168.0.101:11011", "master"]);
        let config = Config::from_args(args);
        assert_eq!(config.mode, Mode::Master);
        assert_eq!(config.bind_address, "192.168.0.101:11011".parse().unwrap());
    }

    #[test]
    fn it_builds_a_default_agent_config() {
        let args = Config::matches()
            .get_matches_from(vec!["mjolnird", "agent", "--master=192.168.0.100:11011"]);
        let config = Config::from_args(args);
        assert_eq!(
            config.mode,
            Mode::Agent(vec!["192.168.0.100:11011".parse().unwrap()])
        );
        assert_eq!(config.bind_address, "0.0.0.0:11012".parse().unwrap());
    }

    #[test]
    fn it_builds_an_agent_config() {
        let args = Config::matches().get_matches_from(vec![
            "mjolnird",
            "--bind=192.168.0.101:11012",
            "agent",
            "--master=192.168.0.100:11011",
        ]);
        let config = Config::from_args(args);
        assert_eq!(
            config.mode,
            Mode::Agent(vec!["192.168.0.100:11011".parse().unwrap()])
        );
        assert_eq!(config.bind_address, "192.168.0.101:11012".parse().unwrap());
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_address: SocketAddr,
    pub mode: Mode,
    pub plugin_path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    Agent(Vec<SocketAddr>),
    Master,
}

impl<'a, 'b> Config {
    fn matches() -> App<'a, 'b> {
        App::new("Mjölnir")
            .version(crate_version!())
            .author(crate_authors!())
            .arg(
                Arg::with_name("bind")
                    .help("What address:port to bind to")
                    .long("bind")
                    .short("b")
                    .takes_value(true),
            )
            .arg(
                        Arg::with_name("plugins")
                            .help("Path to load plugins from")
                            .long("plugins")
                            .short("p")
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
                            .multiple(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("master")
                    .help("The daemon that controls everything")
                    )
    }

    pub fn get_config() -> Config {
        Config::from_args(Config::matches().get_matches())
    }

    fn from_args(matches: ArgMatches) -> Config {
        let mode = match matches.subcommand() {
            ("master", Some(_master_matches)) => Mode::Master,
            ("agent", Some(agent_matches)) => {
                // This unwrap is safe because we declare masters as required
                let masters = agent_matches
                    .values_of("master")
                    .unwrap()
                    .map(|ip| {
                        ip.parse()
                            .expect(&format!("{} is an invalid address", ip)[..])
                    })
                    .collect();
                Mode::Agent(masters)
            }
            (_, _) => unreachable!(),
        };


        let address = matches
            .value_of("bind")
            .unwrap_or_else(|| match mode {
                Mode::Master => "0.0.0.0:11011",
                Mode::Agent(_) => "0.0.0.0:11012",
            })
            .parse()
            .expect("You provided an invalid bind address");

        let path: Option<PathBuf> = if let Some(p) = matches
            .value_of("plugins") {
                Some(PathBuf::from(p))
            } else {
                xdg::BaseDirectories::with_prefix("mjolnir")
                    .ok()
                    .and_then(|xdg| xdg.create_data_directory("plugins").ok())
            };
        println!("XDG_DATA_DIRS: {:?}", path);
        Config {
            bind_address: address,
            mode: mode,
            plugin_path: path,
        }
    }
}
