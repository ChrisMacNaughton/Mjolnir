// use hyper::Error;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{Read, Write};

use yaml_rust::{YamlLoader};
use yaml_rust::Yaml::Array;

use zmq::{self, Socket, Result as ZmqResult};

use mjolnir::Pipeline;
use config::{Config, Mode};

mod master;
mod agent;


use mjolnir_api::{Alert, Remediation, Operation, parse_from_bytes};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_pipeline_from_yaml() {
        let yaml = r#"pipelines:
    - alert:
        type: alertmanager
        name: disk-full
      action:
        type: clean_disk
        # Optional: 
        # args: [] 
    - alert:
        type: test
      action:
        type: something_else
        args:
        - name=test
        "#;
        let pipelines = load_pipeline_from_yaml(yaml);
        println!("Pipelines: {:?}", pipelines);
        assert_eq!(
            vec![
                Pipeline {
                    trigger: Alert {
                        alert_type: "alertmanager".into(),
                        name: Some("disk-full".into()),
                        source: None
                    },
                    action: Remediation {
                        plugin: "clean_disk".into(),
                        target: None,
                        args: vec![]
                    }
                }, Pipeline {
                    trigger: Alert {
                        alert_type: "test".into(),
                        name: None,
                        source: None
                    }, action: Remediation {
                        plugin: "something_else".into(),
                        target: None,
                        args: vec!["name=test".into()] } }
            ],
            pipelines
        )
    }
}

pub fn bind(config: Config) -> ZmqResult<()> {
    match config.mode.clone() {
        Mode::Agent(masters) => agent::Agent::bind(config, masters),
        Mode::Master => master::Master::bind(config),
    }
}

fn load_pipeline(config: &Config) -> Vec<Pipeline> {
    let mut config_file_path = config.config_path.clone();
    config_file_path.push("pipelines.yaml");
    let mut s = String::new();
    match File::open(&config_file_path) {
        Ok(mut f) => {
            let _ = f.read_to_string(&mut s);
        },
        Err(e) => panic!("Couldn't open pipeline config file ({}): {:?}", config_file_path.display(), e),
    }
    
    load_pipeline_from_yaml(&s)
}

fn load_pipeline_from_yaml(yaml: &str) -> Vec<Pipeline> {
    let mut v = vec![];
    match YamlLoader::load_from_str(yaml) {
        Ok(yaml) => {
            // println!("Yaml is: {:?}", yaml);
            if let Some(yaml) = yaml.get(0) {
                match yaml["pipelines"] {
                    Array(ref pipelines) => {
                        // println!("Pipelines: {:?}", pipelines);
                        for pipeline in pipelines.iter() {
                            // println!("pipeline: {:?}", pipeline);
                            let alert_yaml = &pipeline["alert"];
                            let alert = Alert {
                                alert_type: alert_yaml["type"].as_str().expect("Couldn't parse the yaml into a pipeline").into(),
                                name: alert_yaml["name"].as_str().map(|a| Some(a.into())).unwrap_or(None),
                                source: None,
                            };
                            let remediation_yaml = &pipeline["action"];
                            let args = match remediation_yaml["args"] {
                                Array(ref args) => {
                                    args
                                        .iter()
                                        .map(|a| a.as_str())
                                        .filter(|a| a.is_some())
                                        .map(|a| a.unwrap().into())
                                        .collect()
                                },
                                _ => vec![]
                            };
                            let remediation = Remediation {
                                plugin: remediation_yaml["type"].as_str().unwrap().into(),
                                target: None,
                                args: args,
                            };
                            v.push(
                                Pipeline {
                                    trigger: alert,
                                    action: remediation,
                                }
                            )
                        }
                    },
                    _ => panic!("Invalid type for pipelines"),
                }
            }
        },
        Err(e) => println!("Had a problem loading yaml config: {:?}", e),
    }
    v
}

fn server_pubkey(config: &Config) -> String {
    let server_pubkey = {
        let mut pubkey_path = config.config_path.clone();
        pubkey_path.push("ecpubkey.pem");
        if let Ok(mut file) = File::open(&pubkey_path) {
            let mut key = String::new();
            let _ = file.read_to_string(&mut key);
            key
        } else {
            panic!("You need to supply a server's public key, cannot continue");
        }
    };
    server_pubkey
}

fn connect(host: &str, port: u16, server_publickey: &str) -> ZmqResult<Socket> {
    // println!("Starting zmq sender with version({:?})", zmq::version());
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ)?;
    let client_keypair = zmq::CurveKeyPair::new()?;

    requester.set_curve_serverkey(server_publickey)?;
    requester.set_curve_publickey(&client_keypair.public_key)?;
    requester.set_curve_secretkey(&client_keypair.secret_key)?;
    // println!("Connecting to tcp://{}:{}", host, port);
    assert!(
        requester
            .connect(&format!("tcp://{}:{}", host, port))
            .is_ok()
    );
    // println!("Client mechanism: {:?}", requester.get_mechanism());

    Ok(requester)
}

fn setup_curve(s: &mut Socket, config: &Config) -> ZmqResult<()> {
    // will raise EINVAL if not linked against libsodium
    // The ubuntu package is linked so this shouldn't fail
    s.set_curve_server(true)?;
    let mut pubkey_path = config.config_path.clone();
    pubkey_path.push("ecpubkey.pem");
    let mut key_path = config.config_path.clone();
    key_path.push("ecpubkey.key");
    if let Ok(mut file) = File::open(&key_path) {
        let mut key = String::new();
        let _ = file.read_to_string(&mut key);
        s.set_curve_secretkey(&key)?;
    } else {
        // println!("Creating new curve keypair");
        let keypair = zmq::CurveKeyPair::new()?;
        s.set_curve_secretkey(&keypair.secret_key)?;

        let mut f = File::create(pubkey_path).unwrap();
        f.write(keypair.public_key.as_bytes()).unwrap();
        let mut f = File::create(key_path).unwrap();
        f.write(keypair.secret_key.as_bytes()).unwrap();
    }

    // println!("Server mechanism: {:?}", s.get_mechanism());
    // println!("Curve server: {:?}", s.is_curve_server());

    Ok(())
}

/*
Server that manages disks
*/
fn zmq_listen(
    config: &Config,
    callback: Box<Fn(Operation, &Socket) -> ZmqResult<()>>,
) -> ZmqResult<()> {
    println!("Starting zmq listener with version({:?})", zmq::version());
    let context = zmq::Context::new();
    let mut responder = context.socket(zmq::REP)?;

    println!("Listening on {}", config.zmq_address);
    // Fail to start if this fails
    setup_curve(&mut responder, config)?;
    assert!(responder.bind(&config.zmq_address).is_ok());
    println!("Going into the zmq loop");
    let duration = Duration::from_millis(10);
    loop {
        match responder.recv_bytes(0) {
            Ok(msg) => {
                // println!("Got msg len: {}", msg.len());
                // println!("Parsing msg {:?} as hex", msg);
                let operation = match parse_from_bytes::<Operation>(&msg) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        println!("Failed to parse_from_bytes {:?}.  Ignoring request", e);
                        continue;
                    }
                };
                // println!("Operation is: {:?}", operation);
                callback(operation, &responder)?
            }
            Err(e) => {
                println!("Failed to recieve bytes: {:?}", e);
                return Err(e);
            }
        }
        //.expect("Failed to recieve bytes?");


        thread::sleep(duration);
    }
}
