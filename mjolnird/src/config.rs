use std::net::{SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use clap::{App, Arg, ArgMatches, SubCommand};
use xdg;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_a_default_master_config() {
        let args = Config::matches().get_matches_from(vec!["mjolnird", "--ip=127.0.0.1", "master"]);
        let config = Config::from_args(args);
        assert_eq!(config.mode, Mode::Master);
        assert_eq!(config.bind_address, "0.0.0.0:11011".parse().unwrap());
    }

    #[test]
    fn it_builds_a_master_config() {
        let args = Config::matches()
            .get_matches_from(vec!["mjolnird", "--ip=127.0.0.1", "--bind=192.168.0.101:11011", "master"]);
        let config = Config::from_args(args);
        assert_eq!(config.mode, Mode::Master);
        assert_eq!(config.bind_address, "192.168.0.101:11011".parse().unwrap());
    }

    #[test]
    fn it_builds_a_master_config_with_plugin_path() {
        let args = Config::matches()
            .get_matches_from(vec!["mjolnird", "--bind=192.168.0.101:11011", "--plugins=/usr/local/share", "--ip=127.0.0.1", "master"]);
        let config = Config::from_args(args);
        assert_eq!(config.mode, Mode::Master);
        assert_eq!(config.bind_address, "192.168.0.101:11011".parse().unwrap());
        assert_eq!(config.plugin_path, PathBuf::from("/usr/local/share"));
        
    }

    #[test]
    fn it_builds_a_default_agent_config() {
        let args = Config::matches()
            .get_matches_from(vec!["mjolnird", "--ip=127.0.0.1", "agent", "--master=192.168.0.100:11011"]);
        let config = Config::from_args(args);
        assert_eq!(
            config.mode,
            Mode::Agent(vec!["192.168.0.100:11011".parse().unwrap()])
        );
    }

    #[test]
    fn it_builds_an_agent_config() {
        let args = Config::matches().get_matches_from(vec![
            "mjolnird",
            "--ip=127.0.0.1",
            "agent",
            "--master=192.168.0.100:11011",
        ]);
        let config = Config::from_args(args);
        assert_eq!(
            config.mode,
            Mode::Agent(vec!["192.168.0.100:11011".parse().unwrap()])
        );
    }

    #[test]
    fn it_can_parse_a_master_with_defaults() {
        let input = "127.0.0.1";
        let master: Master = input.parse().unwrap();
        assert_eq!(master, Master{
            ip: "127.0.0.1".into(),
            http_port: 11011,
            zmq_port: 12011
        });
    }

    #[test]
    fn it_can_parse_a_master_with_http_defaults() {
        let input = "127.0.0.1::8080";
        let master: Master = input.parse().unwrap();
        assert_eq!(master, Master{
            ip: "127.0.0.1".into(),
            http_port: 11011,
            zmq_port: 8080
        });
    }

    #[test]
    fn it_can_parse_a_master_with_explicit_zmq_defaults() {
        let input = "127.0.0.1:8080:";
        let master: Master = input.parse().unwrap();
        assert_eq!(master, Master{
            ip: "127.0.0.1".into(),
            http_port: 8080,
            zmq_port: 12011
        });
    }

    #[test]
    fn it_can_parse_a_master_with_implicit_zmq_defaults() {
        let input = "127.0.0.1:8080";
        let master: Master = input.parse().unwrap();
        assert_eq!(master, Master{
            ip: "127.0.0.1".into(),
            http_port: 8080,
            zmq_port: 12011
        });
    }

    #[test]
    fn it_can_parse_a_master_with_no_defaults() {
        let input = "127.0.0.1:8080:9080";
        let master: Master = input.parse().unwrap();
        assert_eq!(master, Master{
            ip: "127.0.0.1".into(),
            http_port: 8080,
            zmq_port: 9080
        });
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_address: SocketAddr,
    pub my_ip: String,
    pub zmq_port: u16,
    pub zmq_address: String,
    pub mode: Mode,
    pub plugin_path: PathBuf,
    pub config_path: PathBuf,
}

#[derive(Clone, Debug,PartialEq)]
pub struct Master {
    pub ip: String,
    pub http_port: u16,
    pub zmq_port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddrParseError(());

impl FromStr for Master {
    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let ip = match parts.next() {
            Some(ip) => ip,
            None => return Err(AddrParseError(()))
        };
        let http_port = if let Some(port) = parts.next() {
            if port == "" {
                11011
            } else {
                match port.parse() {
                    Ok(p) => p,
                    Err(_) => return Err(AddrParseError(()))
                }
            }
        } else {
            11011
        };
        let zmq_port = if let Some(port) = parts.next() {
            if port == "" {
                12011
            } else {
                match port.parse() {
                    Ok(p) => p,
                    Err(_) => return Err(AddrParseError(()))
                }
            }
        } else {
            12011
        };
        Ok(Master {
            ip: ip.into(),
            http_port:  http_port,
            zmq_port: zmq_port,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    Agent(Vec<Master>),
    Master,
}

impl<'a, 'b> Config {
    fn matches() -> App<'a, 'b> {
        App::new("Mjölnir")
            .version(crate_version!())
            .author(crate_authors!())
            .arg(
                Arg::with_name("bind")
                    .help("What address:port to bind to for http")
                    .long("bind")
                    .short("b")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("zmq")
                    .help("What address:port to bind to for zeromq")
                    .long("zmq")
                    .short("z")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("config path")
                    .help("Path to save configuration in")
                    .long("config")
                    .short("c")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("plugins")
                    .help("Path to load plugins from")
                    .long("plugins")
                    .short("p")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("my_ip")
                    .help("The IP that another server can reach me on")
                    .long("ip")
                    .short("i")
                    .required(true)
                    .takes_value(true)
            )
            .subcommand(
                SubCommand::with_name("agent")
                    .help("THe machine agent that runs on every machine")
                    .arg(
                        Arg::with_name("master")
                            .help("IP Address[es] of the master, in the format x.x.x.x:HTTP_PORT:ZMQ_PORT.
                                  If either of the ports are empty (x.x.x.x::ZMQ_PORT), then the defaults
                                  will be used. At a minimum, the IP address is required")
                            .long("master")
                            .short("m")
                            .required(true)
                            .takes_value(true)
                            .multiple(true),
                    )
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

        // only relevant for the master
        let address = matches
            .value_of("bind")
            .unwrap_or("0.0.0.0:11011")
            .parse()
            .expect("You provided an invalid bind address");

        let zmq_address = format!("tcp://{}", matches
            .value_of("bind")
            .unwrap_or_else(|| match mode {
                Mode::Master => "0.0.0.0:12011",
                Mode::Agent(_) => "0.0.0.0:12012",
            }));
        
        let zmq_port = zmq_address.split(":").last().unwrap().parse().unwrap();

        let path: PathBuf = if let Some(p) = matches
            .value_of("plugins") {
                Some(PathBuf::from(p))
            } else {
                xdg::BaseDirectories::with_prefix("mjolnir")
                    .ok()
                    .and_then(|xdg| xdg.create_data_directory("plugins").ok())
            }.expect("Couldn't determine plugin path, please specify one");
        // println!("XDG_DATA_DIRS: {:?}", path);
        let config_path: PathBuf = if let Some(p) = matches
            .value_of("config") {
                Some(PathBuf::from(p))
            } else {
                xdg::BaseDirectories::with_prefix("mjolnir")
                    .ok()
                    .and_then(|xdg| xdg.create_config_directory("").ok())
            }.expect("Couldn't determine config path, please specify one");
        Config {
            bind_address: address,
            zmq_address: zmq_address,
            my_ip: matches.value_of("my_ip").unwrap().into(),
            mode: mode,
            plugin_path: path,
            config_path: config_path,
            zmq_port: zmq_port,
        }
    }
}
