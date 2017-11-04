use std::net::SocketAddr;
// use std::sync::{Arc, Mutex};

// use futures;
// use futures::future::Future;
// use futures::Future;
use std::fs::{File};
use std::io::{Read, Write};

use hostname::get_hostname;
use hyper;
// use hyper::header::ContentLength;
// use hyper::server::{Http, Request, Response, Service};

// use hyper::Client;

// use tokio_core::reactor::Core;

use mjolnir_api::{Operation, OperationType as OpType, Register, parse_from_bytes};

use protobuf::Message as ProtobufMsg;

use zmq::{self, Message, Socket, Result as ZmqResult};

use config::{Config, Master};
use server::{connect, zmq_listen};

#[derive(Clone)]
pub struct Agent {
    masters: Vec<Master>,
    pubkey: String,
    config: Config,
}

impl Agent {
    pub fn bind(config: Config, masters: Vec<Master>) -> ZmqResult<()> {
        // lets get registered!
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
        let agent = Agent {
            masters: masters,
            pubkey: server_pubkey.into(),
            config: config,
        };

        let _ = agent.register();
        let _ = agent.listen();
        Ok(())
    }

    fn listen(&self) -> ZmqResult<()> {
        zmq_listen(&self.config, Box::new(|operation, responder| {
            match operation.get_operation_type() {
                OpType::PING => {
                    let mut o = Operation::new();
                    println!("Creating pong");
                    o.set_operation_type(OpType::PONG);
                    o.set_ping_id(operation.get_ping_id());
                    let encoded = o.write_to_bytes().unwrap();
                    let msg = Message::from_slice(&encoded)?;
                    responder.send_msg(msg, 0)?;
                }
                _ => {
                    println!("Not quite handling {:?} yet", operation);
                }
            }
            Ok(())
        }))
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
        let mut register = Register::new();
        register.set_hostname(get_hostname().unwrap());
        register.set_ip(self.config.my_ip.clone());
        register.set_port(self.config.zmq_port as i32);
        o.set_register(register);
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
            },
            Err(e) => {
                println!("Failed to recieve bytes: {:?}", e);
                return Err(e);
            },
        }
        Ok(())
    }
}
