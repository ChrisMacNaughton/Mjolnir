// use futures;
// use futures::future::Future;
// use futures::Future;
use std::fs;
// use std::os::unix::OpenOptionsExt;
use std::os::unix::fs::OpenOptionsExt;
use std::io;
use std::process::Command;
use std::sync::{Arc, RwLock};
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

use uuid::Uuid;
use zmq::{Message, Result as ZmqResult, Socket};

use config::{Config, Master};
use server::{connect, get_master_pubkey, run_plugin, server_pubkey, zmq_listen};

#[derive(Clone)]
pub struct Agent {
    // masters: Arc<Vec<Master>>,
    server_pubkey: String,
    my_pubkey: String,
    config: Config,
    my_id: Uuid,
}

impl Agent {
    pub fn bind(config: Config) -> ZmqResult<()> {

        let background_config = config.clone();

        let agent = Agent {
            my_pubkey: server_pubkey(&config).into(),
            config: config,
            server_pubkey: server_pubkey(&background_config),
            my_id: Uuid::new_v4(),
        };

        let _ = fs::create_dir_all(&agent.config.plugin_path);
        let _ = agent.register();
        let ping_duration = Duration::from_millis(500);
        let masters = agent.config.masters.clone();
        thread::spawn(move|| {
            loop {
                for master in masters.iter() {
                    let server_key = get_master_pubkey(&master).expect("Couldn't load the master's public key");
                    match connect(&master.ip, master.zmq_port, &server_key){
                        Ok(socket) => {
                            let mut o = Operation::new();
                            o.set_operation_type(OpType::PING);

                            let encoded = o.write_to_bytes().unwrap();
                            let msg = Message::from_slice(&encoded).unwrap();
                            match socket.send_msg(msg, 0) {
                                Ok(_s) => {},
                                Err(e) => warn!("Problem snding ping: {:?}", e)
                            }
                        }
                        Err(e) => warn!("problem connecting to socket: {:?}", e),
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
        let masters = config.masters.clone();
        let server_pubkey = self.server_pubkey.clone();
        zmq_listen(
            &Arc::new(RwLock::new(config.clone())),
            Box::new(move|operation, responder| {
                match operation.get_operation_type() {
                    OpType::PING => {
                        let mut o = Operation::new();
                        o.set_operation_type(OpType::PONG);
                        o.set_ping_id(operation.get_ping_id());
                        let encoded = o.write_to_bytes().unwrap();
                        let msg = Message::from_slice(&encoded)?;
                        responder.send_msg(msg, 0)?;
                    }
                    OpType::REMEDIATE => {
                        let mut o = Operation::new();
                        o.set_operation_type(OpType::ACK);

                        let encoded = o.write_to_bytes().unwrap();
                        let msg = Message::from_slice(&encoded)?;
                        responder.send_msg(msg, 0)?;

                        let remediation: Remediation = operation.get_remediate().into();
                        debug!("About to try to remediate {:?}", remediation);
                        let res = remediate(remediation, &config, &masters);

                        debug!("Result: {:?}", res);

                        match connect(&masters[0].ip, masters[0].zmq_port, &server_pubkey){
                            Ok(socket) => {
                                let mut o = Operation::new();
                                o.set_operation_type(OpType::REMEDIATION_RESULT);
                                o.set_result(res.into());
                                let encoded = o.write_to_bytes().unwrap();
                                let msg = Message::from_slice(&encoded).unwrap();
                                match socket.send_msg(msg, 0) {
                                    Ok(_s) => {},
                                    Err(e) => warn!("Problem sending result: {:?}", e)
                                }
                            }
                            Err(e) => warn!("problem connecting to socket: {:?}", e),
                        }
                        
                    }
                    _ => {
                        debug!("Not quite handling {:?} yet", operation);

                        let mut o = Operation::new();
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
        for master in &self.config.masters {
            let socket = match connect(&master.ip, master.zmq_port, &self.server_pubkey) {
                Ok(s) => s,
                Err(e) => {
                    error!("Error connecting to socket: {:?}", e);
                    return Err(e);
                }
            };
            // TODO : when we persist agents on the master side, this can bail on the first success
            self.register_msg(&socket)?;

            match socket.recv_bytes(0) {
                Ok(msg) => {
                    trace!("Got msg len: {}", msg.len());
                    trace!("Parsing msg {:?} as hex", msg);
                    let operation = match parse_from_bytes::<Operation>(&msg) {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            warn!("Failed to parse_from_bytes {:?}.  Ignoring request", e);
                            // TODO: Proper error handling
                            return Ok(());
                        }
                    };
                    debug!("Operation is: {:?}", operation);
                    match operation.get_operation_type() {
                        OpType::ACK => {
                            trace!("got our ACK!");
                        }
                        OpType::NACK => {
                            error!("We supplied a bad secret");
                            panic!("Misconfigured shared secret");
                        }
                        _ => {
                            debug!("Not quite handling {:?} yet", operation);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to recieve bytes: {:?}", e);
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    fn register_msg(&self, socket: &Socket) -> ZmqResult<()>{
        let mut o = Operation::new();
        debug!("Creating  operation request");
        o.set_operation_type(OpType::REGISTER);
        let register = Register::new(
            self.config.bind_ip.clone(),
            self.config.zmq_port,
            get_hostname().unwrap(),
            self.config.secret.clone(),
            self.my_pubkey.clone(),
            self.my_id.clone(),
        );
        o.set_register(register.into());
        let encoded = o.write_to_bytes().unwrap();
        let msg = Message::from_slice(&encoded)?;
        trace!("Sending message");
        socket.send_msg(msg, 0)
    }
}


fn remediate(remediation: Remediation, config: &Config, masters: &Vec<Master>) -> RemediationResult {
    let plugin_path = {
        let mut plugin_path = config.plugin_path.clone();
        plugin_path.push(&remediation.plugin);
        plugin_path
    };
    if ! plugin_path.exists() {
        let master = if let Some(master) = masters.first() {
            master
        } else {
            return RemediationResult::new().err("Couldn't find the masters").with_alert(remediation.alert.unwrap().increment())
        };
        if let Ok(mut resp) = reqwest::get(&format!("http://{}:{}/plugin/{}", master.ip, master.http_port, remediation.plugin)) {
            let status = resp.status();
            if !status.is_success() {
                return RemediationResult::new().err(format!("couldn't download {} : {:?}", plugin_path.display(), status)).with_alert(remediation.alert.unwrap().increment())
            }
            match fs::OpenOptions::new()
                .create(true)
                .write(true)
                .mode(0o770)
                .open(&plugin_path) {
                Ok(mut f) => {
                    if let Err(e) = io::copy(&mut resp, &mut f) {
                        return RemediationResult::new().err(format!("couldn't download {} : {:?}", plugin_path.display(), e)).with_alert(remediation.alert.unwrap().increment())
                    }
                },
                Err(e) => return RemediationResult::new().err(format!("couldn't create {} : {:?}", plugin_path.display(), e)).with_alert(remediation.alert.unwrap().increment())
            };
        } else {
            return RemediationResult::new().err(format!("Couldn't fetch the plugin from the master at {}:{}", master.ip,  master.http_port)).with_alert(remediation.alert.unwrap().increment())
        }
    }
    let plugin = match Command::new(&plugin_path).output() {
        Ok(output) => {
            match PluginEntry::try_from(
                &output.stdout,
                &plugin_path,
            ) {
                Ok(plugin) => plugin,
                Err(e) => return RemediationResult::new().err(format!("Had a problem loading plugin at {}: {:?}", plugin_path.display(), e)).with_alert(remediation.alert.unwrap().increment())
            }
        }
        Err(e) => return RemediationResult::new().err(format!("Had a problem loading plugin at {}: {:?}", plugin_path.display(), e)).with_alert(remediation.alert.unwrap().increment())
    };

    let mut res = run_plugin(&plugin, &remediation);
    if res.result.is_err() {
        res = res.with_alert(remediation.alert.unwrap().increment());
    }
    res
}