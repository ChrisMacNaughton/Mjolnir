// use futures;
// use futures::future::Future;
// use futures::Future;
use std::fs;
// use std::os::unix::OpenOptionsExt;
use std::os::unix::fs::OpenOptionsExt;
use std::io;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use std::thread;

use hostname::get_hostname;
// use hyper;
// use hyper::header::ContentLength;
// use hyper::server::{Http, Request, Response, Service};

// use hyper::Client;

use reqwest;

// use tokio_core::reactor::Core;

use mjolnir_api::{Operation, OperationType as OpType, Register, parse_from_bytes, PluginEntry, Remediation, RemediationResult};

use protobuf::Message as ProtobufMsg;

use zmq::{Message, Result as ZmqResult};

use config::{Config, Master};
use server::{connect, server_pubkey, zmq_listen};

#[derive(Clone)]
pub struct Agent {
    masters: Arc<Vec<Master>>,
    pubkey: String,
    config: Config,
}

impl Agent {
    pub fn bind(config: Config, masters: Vec<Master>) -> ZmqResult<()> {

        let background_config = config.clone();
        let agent = Agent {
            masters: Arc::new(masters),
            pubkey: server_pubkey(&config).into(),
            config: config,
        };

        let _ = fs::create_dir_all(&agent.config.plugin_path);
        let _ = agent.register();
        let ping_duration = Duration::from_millis(500);
        let masters = agent.masters.clone();
        thread::spawn(move|| {
            let server_pubkey = server_pubkey(&background_config);
            loop {
                for master in masters.iter() {
                    match connect(&master.ip, master.zmq_port, &server_pubkey){
                        Ok(socket) => {
                            let mut o = Operation::new();
                            // println!("Creating PING");
                            o.set_operation_type(OpType::PING);

                            let encoded = o.write_to_bytes().unwrap();
                            let msg = Message::from_slice(&encoded).unwrap();
                            match socket.send_msg(msg, 0) {
                                Ok(_s) => {},
                                Err(e) => println!("Problem snding ping: {:?}", e)
                            }
                        }
                        Err(e) => println!("problem connecting to socket: {:?}", e),
                    }
                }
            
                thread::sleep(ping_duration);
            }
        });
        let _ = agent.listen();
        Ok(())
    }

    fn listen(&self) -> ZmqResult<()> {
        let config = self.config.clone();
        let masters = self.masters.clone();
        let server_pubkey = self.pubkey.clone();
        zmq_listen(
            &self.config,
            Box::new(move|operation, responder| {
                match operation.get_operation_type() {
                    OpType::PING => {
                        let mut o = Operation::new();
                        // println!("Creating pong");
                        o.set_operation_type(OpType::PONG);
                        o.set_ping_id(operation.get_ping_id());
                        let encoded = o.write_to_bytes().unwrap();
                        let msg = Message::from_slice(&encoded)?;
                        responder.send_msg(msg, 0)?;
                    }
                    OpType::REMEDIATE => {
                        let mut o = Operation::new();
                        println!("Creating ack for {:?}", operation.get_operation_type());
                        o.set_operation_type(OpType::ACK);

                        let encoded = o.write_to_bytes().unwrap();
                        let msg = Message::from_slice(&encoded)?;
                        responder.send_msg(msg, 0)?;

                        let remediation: Remediation = operation.get_remediate().into();
                        println!("About to try to remediate {:?}", remediation);
                        let res = remediate(remediation, &config, &masters);

                        println!("Result: {:?}", res);

                        match connect(&masters[0].ip, masters[0].zmq_port, &server_pubkey){
                            Ok(socket) => {
                                let mut o = Operation::new();
                                // println!("Creating PING");
                                o.set_operation_type(OpType::REMEDIATION_RESULT);
                                o.set_result(res.into());
                                let encoded = o.write_to_bytes().unwrap();
                                let msg = Message::from_slice(&encoded).unwrap();
                                match socket.send_msg(msg, 0) {
                                    Ok(_s) => {},
                                    Err(e) => println!("Problem snding result: {:?}", e)
                                }
                            }
                            Err(e) => println!("problem connecting to socket: {:?}", e),
                        }
                        
                    }
                    _ => {
                        println!("Not quite handling {:?} yet", operation);

                        let mut o = Operation::new();
                        println!("Creating ack for {:?}", operation.get_operation_type());
                        o.set_operation_type(OpType::ACK);

                        let encoded = o.write_to_bytes().unwrap();
                        let msg = Message::from_slice(&encoded)?;
                        responder.send_msg(msg, 0)?;
                    }
                }
                Ok(())
            }),
        )
    }

    fn register(&self) -> ZmqResult<()> {
        // register with the master!
        let master = &self.masters[0];
        let socket = match connect(&master.ip, master.zmq_port, &self.pubkey) {
            Ok(s) => s,
            Err(e) => {
                println!("Error connecting to socket: {:?}", e);
                return Err(e);
            }
        };

        let mut o = Operation::new();
        println!("Creating  operation request");
        o.set_operation_type(OpType::REGISTER);
        let register = Register::new(
            self.config.my_ip.clone(),
            self.config.zmq_port,
            get_hostname().unwrap(),
        );
        o.set_register(register.into());
        let encoded = o.write_to_bytes().unwrap();
        let msg = Message::from_slice(&encoded)?;
        println!("Sending message");
        socket.send_msg(msg, 0)?;
        match socket.recv_bytes(0) {
            Ok(msg) => {
                println!("Got msg len: {}", msg.len());
                println!("Parsing msg {:?} as hex", msg);
                let operation = match parse_from_bytes::<Operation>(&msg) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        println!("Failed to parse_from_bytes {:?}.  Ignoring request", e);
                        // TODO: Proper error handling
                        return Ok(());
                    }
                };
                println!("Operation is: {:?}", operation);
                match operation.get_operation_type() {
                    OpType::ACK => {
                        println!("got our ACK!");
                    }
                    _ => {
                        println!("Not quite handling {:?} yet", operation);
                    }
                }
            }
            Err(e) => {
                println!("Failed to recieve bytes: {:?}", e);
                return Err(e);
            }
        }
        Ok(())
    }
}


fn remediate(remediation: Remediation, config: &Config, masters: &Arc<Vec<Master>>) -> RemediationResult {
    let plugin_path = {
        let mut plugin_path = config.plugin_path.clone();
        plugin_path.push(&remediation.plugin);
        plugin_path
    };
    if ! plugin_path.exists() {
        let master = if let Some(master) = masters.first() {
            master
        } else {
            return RemediationResult::new().err("Couldn't find the masters")
        };
        if let Ok(mut resp) = reqwest::get(&format!("http://{}:{}/plugin/{}", master.ip, master.http_port, remediation.plugin)) {
            let status = resp.status();
            if !status.is_success() {
                return RemediationResult::new().err(format!("couldn't download {} : {:?}", plugin_path.display(), status))
            }
            match fs::OpenOptions::new()
                .create(true)
                .write(true)
                .mode(0o770)
                .open(&plugin_path) {
                Ok(mut f) => {
                    if let Err(e) = io::copy(&mut resp, &mut f) {
                        return RemediationResult::new().err(format!("couldn't download {} : {:?}", plugin_path.display(), e))
                    }
                },
                Err(e) => return RemediationResult::new().err(format!("couldn't create {} : {:?}", plugin_path.display(), e))
            };
        } else {
            return RemediationResult::new().err(format!("Couldn't fetch the plugin from the master at {}:{}", master.ip,  master.http_port))
        }
    }
    let plugin = match Command::new(&plugin_path).output() {
        Ok(output) => {
            match PluginEntry::try_from(
                &output.stdout,
                &plugin_path,
            ) {
                Ok(plugin) => plugin,
                Err(e) => return RemediationResult::new().err(format!("Had a problem loading plugin at {}: {:?}", plugin_path.display(), e))
            }
        }
        Err(e) => return RemediationResult::new().err(format!("Had a problem loading plugin at {}: {:?}", plugin_path.display(), e))
    };

    run_plugin(&plugin, &remediation)
}

fn run_plugin(plugin: &PluginEntry, remediation: &Remediation) -> RemediationResult {
    // println!("Hook is: {:?}", hook);
    let mut cmd = Command::new(&plugin.path);
    cmd.arg(format!("plugin={}", plugin.name));
    // cmd.arg(format!("body={}", body));
    for arg in &remediation.args {
        println!("Adding {} to {:?}", arg, cmd);
        cmd.arg(&arg);
    }
    if let Some(ref alert) = remediation.alert {
        for arg in &alert.args {
            println!("Adding {} to {:?}", arg, cmd);
            cmd.arg(&arg);
        }
    }
    println!("About to run command: {:?}", cmd);
    match cmd.output() {
        Ok(output) => {
            match String::from_utf8(output.stdout) {
                Ok(s) => RemediationResult::from_string(&s),
                Err(e) => RemediationResult::new().err(format!("{:?}", e)),
            }
        }
        Err(e) => RemediationResult::new().err(format!("{:?}", e))
    }
}