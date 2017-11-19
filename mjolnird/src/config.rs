use std::fs::File;
use std::io::Read;
use std::net::{IpAddr};
use std::path::PathBuf;
use std::str::FromStr;

use clap::{App, Arg, ArgMatches, SubCommand};
use toml;
use xdg;

use mjolnir::Pipeline;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_err_with_bad_http_port() {
        let input = "127.0.0.1:wrong:12011";
        assert!(input.parse::<Master>().is_err());
    }

    #[test]
    fn it_returns_err_with_bad_zmq_port() {
        let input = "127.0.0.1:11011:wrong";
        assert!(input.parse::<Master>().is_err());
    }

    #[test]
    fn it_can_parse_a_master_with_defaults() {
        let input = "127.0.0.1";
        let master: Master = input.parse().unwrap();
        assert_eq!(
            master,
            Master {
                ip: "127.0.0.1".into(),
                http_port: 11011,
                zmq_port: 12011,
            }
        );
    }

    #[test]
    fn it_can_parse_a_master_with_http_defaults() {
        let input = "127.0.0.1::8080";
        let master: Master = input.parse().unwrap();
        assert_eq!(
            master,
            Master {
                ip: "127.0.0.1".into(),
                http_port: 11011,
                zmq_port: 8080,
            }
        );
    }

    #[test]
    fn it_can_parse_a_master_with_explicit_zmq_defaults() {
        let input = "127.0.0.1:8080:";
        let master: Master = input.parse().unwrap();
        assert_eq!(
            master,
            Master {
                ip: "127.0.0.1".into(),
                http_port: 8080,
                zmq_port: 12011,
            }
        );
    }

    #[test]
    fn it_can_parse_a_master_with_implicit_zmq_defaults() {
        let input = "127.0.0.1:8080";
        let master: Master = input.parse().unwrap();
        assert_eq!(
            master,
            Master {
                ip: "127.0.0.1".into(),
                http_port: 8080,
                zmq_port: 12011,
            }
        );
    }

    #[test]
    fn it_can_parse_a_master_with_no_defaults() {
        let input = "127.0.0.1:8080:9080";
        let master: Master = input.parse().unwrap();
        assert_eq!(
            master,
            Master {
                ip: "127.0.0.1".into(),
                http_port: 8080,
                zmq_port: 9080,
            }
        );
    }

    #[test]
    fn it_returns_zmq_address() {
        let args = Config::matches().get_matches_from(vec![
            "mjolnird",
            "--config=../examples/configs/mjolnir.toml",
            "master",
        ]);
        let config = Config::from_args(args);
        assert_eq!(config.zmq_address(), "0.0.0.0:12011");
    }

    #[test]
    fn empty_vec() {
        let empty_vec: Vec<Pipeline> = vec![];
        assert_eq!(empty_vec, empty());
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Root {
    mjolnir: ConfigFile,
    #[serde(default="empty")]
    pipelines: Vec<Pipeline>,
}



fn empty() -> Vec<Pipeline> {
    vec![]
}

#[derive(Clone, Debug, Deserialize)]
struct ConfigFile {
    masters: Vec<String>,
    plugin_path: Option<PathBuf>,
    config_path: Option<PathBuf>,
    master: Option<String>,
    agent: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub masters:Vec<Master>,
    pub bind_ip: IpAddr,
    pub http_port: u16,
    pub zmq_port: u16,
    pub mode: Mode,
    pub plugin_path: PathBuf,
    pub config_path: PathBuf,
    pub pipelines: Vec<Pipeline>,
}

#[derive(Clone, Debug, PartialEq)]
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
            None => return Err(AddrParseError(())),
        };
        let http_port = if let Some(port) = parts.next() {
            if port == "" {
                11011
            } else {
                match port.parse() {
                    Ok(p) => p,
                    Err(_) => return Err(AddrParseError(())),
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
                    Err(_) => return Err(AddrParseError(())),
                }
            }
        } else {
            12011
        };
        Ok(Master {
            ip: ip.into(),
            http_port: http_port,
            zmq_port: zmq_port,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Agent,
    Master,
}

impl<'a, 'b> Config {
    pub fn matches() -> App<'a, 'b> {
        App::new("Mjölnir")
            .version(crate_version!())
            .author(crate_authors!())
            .arg(
                Arg::with_name("config")
                    .help("What is the path to my config file")
                    .long("config")
                    .short("c")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("debug")
                    .help("How verbose to log at")
                    .long("debug")
                    .short("d")
                    .multiple(true)
            )
            .subcommand(
                SubCommand::with_name("agent")
                    .help("THe machine agent that runs on every machine")
            )
            .subcommand(SubCommand::with_name("master")
                .help("The daemon that controls everything")
            )
    }

    pub fn zmq_address(&self) -> String {
        format!("{}:{}", self.bind_ip, self.zmq_port)
    }

    pub fn get_config() -> Config {
        Config::from_args(Config::matches().get_matches())
    }

    pub fn from_args(matches: ArgMatches) -> Config {
        let mode = match matches.subcommand() {
            ("master", Some(_master_matches)) => Mode::Master,
            ("agent", Some(_agent_matches)) => Mode::Agent,
            (_, _) => unreachable!(),
        };

        let path: PathBuf = {
            if let Some(p) = matches.value_of("config") {
                PathBuf::from(p)
            } else {
                let mut p = xdg::BaseDirectories::with_prefix("mjolnir").ok().and_then(
                    |xdg| {
                        xdg.create_data_directory("").ok()
                    },
                ).expect("Couldn't determine plugin path, please specify one");

                p.push("config.toml");
                p
            }
        };

        println!("Trying to load config from {}", path.display());

        let config_raw = match File::open(path) {
            Ok(mut f) => {
                let mut s = String::new();
                let _ = f.read_to_string(&mut s);
                s
            }
            Err(e) => {
                panic!("Err: {:?}", e);
            }
        };

        let root: Root = match toml::from_str(&config_raw) {
            Ok(a) => a,
            Err(e) => panic!("Couldn't parse your config: {:?}", e),
        };

        let config_file = root.mjolnir;

        let plugin_path: PathBuf = if let Some(p) = config_file.plugin_path {
            Some(PathBuf::from(p))
        } else {
            xdg::BaseDirectories::with_prefix("mjolnir").ok().and_then(
                |xdg| {
                    xdg.create_data_directory("plugins").ok()
                },
            )
        }.expect("Couldn't determine plugin path, please specify one");
        // // println!("XDG_DATA_DIRS: {:?}", path);
        let config_path: PathBuf = if let Some(p) = config_file.config_path {
            Some(PathBuf::from(p))
        } else {
            xdg::BaseDirectories::with_prefix("mjolnir").ok().and_then(
                |xdg| {
                    xdg.create_config_directory("").ok()
                },
            )
        }.expect("Couldn't determine config path, please specify one");

        let me = match mode {
            Mode::Master => {
                if let Some(me) = config_file.master {
                    Master::from_str(&me).expect(&format!("Couldn't parse my details from {}", me))
                } else {
                    Master {
                        ip: "0.0.0.0".into(),
                        http_port: 11011,
                        zmq_port: 12011,
                    }
                }
            },
            Mode::Agent => {
                if let Some(me) = config_file.agent {
                    Master::from_str(&me).expect(&format!("Couldn't parse my details from {}", me))
                } else {
                    Master {
                        ip: "0.0.0.0".into(),
                        http_port: 11012,
                        zmq_port: 12012,
                    }
                }
            },
        };
        Config {
            mode: mode,
            masters: config_file.masters.iter().map(|a| Master::from_str(a).expect(&format!("Couldn't parse {} into IP:HTTP_PORT:ZMQ_PORT", a))).collect(),
            bind_ip: IpAddr::from_str(&me.ip).expect(&format!("Couldn't parse IP from {}", me.ip)),
            http_port: match mode {
                Mode::Master => me.http_port,
                Mode::Agent => me.http_port + 1,
            },
            zmq_port: match mode {
                Mode::Master => me.zmq_port,
                Mode::Agent => me.zmq_port + 1,
            },
            plugin_path: plugin_path,
            config_path: config_path,
            pipelines: root.pipelines,
        }
    }
}
