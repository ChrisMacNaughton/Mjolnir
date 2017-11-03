use std::time::Duration;
use std::fs::{read_dir, File};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

use futures;
// use futures::future::Future;
use futures::{Future, Stream};

use hyper;
use hyper::server::{Http, Request, Response, Service};

use hyper::{Body, Chunk, Method, StatusCode};

use zmq::{self, Message, Socket, Result as ZmqResult};

use protobuf::Message as ProtobufMsg;

use mjolnir::PluginEntry;

use mjolnir_api::{Operation, OperationType as OpType, parse_from_bytes};
use server::zmq_listen;
use config::Config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_webhook() {
        let plugin = PluginEntry {
            name: "test-name".into(),
            author: "test author".into(),
            version: "test version".into(),
            webhook: true,
            alerts: vec![],
            remediations: vec![],
            path: PathBuf::from("/bin/echo"),
        };

        let body = process_webhook(plugin, "test".into());
        assert_eq!(body, "plugin=test-name body=test\n")
    }
}
#[derive(Clone, Debug, Default)]
pub struct Master {
    agents: Arc<Mutex<Vec<SocketAddr>>>,
    plugins: Vec<PluginEntry>,
    plugin_path: Option<PathBuf>,
}

impl Service for Master {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    // type Response = Response;
    type Response = Response<Box<Stream<Item = Chunk, Error = Self::Error>>>;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        self.route(req)
    }
}

fn hello(
    req: Request,
) -> Box<
    Future<Item = Response<Box<Stream<Item = Chunk, Error = hyper::Error>>>, Error = hyper::Error>,
> {
    let phrase = "Hello, from Master";
    let mut response = Response::new();
    println!("Received request: {} {}", req.method(), req.path());
    let body: Box<Stream<Item = _, Error = _>> = Box::new(Body::from(phrase));
    response.set_body(body);
    // response.set_body(phrase);
    Box::new(futures::future::ok(response))
}

fn webhook(
    name: &str,
    req: Request,
    plugins: &Vec<PluginEntry>,
) -> Box<
    Future<Item = Response<Box<Stream<Item = Chunk, Error = hyper::Error>>>, Error = hyper::Error>,
> {
    println!("Responding to webook {} at {}", name, req.path());
    // let plugins = plugins.clone();
    let hook = plugins
        .iter()
        .filter(|wh| wh.webhook)
        .filter(|wh| wh.name == name)
        .nth(0)
        .map(|p| p.clone());
    // let hook: Option<PluginEntry> = *hook.clone();

    Box::new(req.body().concat2().map(move |body| {
        // let plugins = plugins.clone();
        let body: Box<Stream<Item = _, Error = _>> = if let Some(hook) = hook {
            match String::from_utf8(body.to_vec()) {
                Ok(s) => Box::new(Body::from(process_webhook(hook, s))),
                Err(_) => Box::new(Body::from("Invalid Body")),
            }
        // println!("Body is: {:?}", body);
        // cmd.arg(hook.name);
        // hook.args.each
        } else {
            Box::new(Body::from("Unknown Webhook"))
        };
        let mut response: Response<Box<Stream<Item = Chunk, Error = hyper::Error>>> =
            Response::new();

        response.set_body(body);
        response
    }))
}

fn process_webhook(hook: PluginEntry, body: String) -> String {
    println!("Hook is: {:?}", hook);
    let mut cmd = Command::new(hook.path);
    cmd.arg(format!("plugin={}", hook.name));
    cmd.arg(format!("body={}", body));
    if let Ok(output) = cmd.output() {
        match String::from_utf8(output.stdout) {
            Ok(s) => s,
            Err(_) => "".into(),
        }
    } else {
        "Ok".into()
    }
}

impl Master {
    pub fn bind(config: Config) -> ZmqResult<()> {
        let master = Master::default()
            .with_plugin_path(config.plugin_path.clone())
            .load_plugins();

        let http_config = config.clone();

        // OH MY GOD THE PAIN TO KEEP THE RIGHT THING ALIVE
        let closure_master = master.clone();
        thread::spawn(move||{
            let master_server = move || Ok(closure_master.clone());
            let server = Http::new().bind(&http_config.bind_address, master_server)?;
            server.run()
        });
        // let _ = master.zmq_listen(&config)?;
        let _ = master.setup_zmq(&config)?;
        thread::park();
        Ok(())
    }

    fn setup_zmq(&self, config: &Config) -> ZmqResult<()>{
        zmq_listen(config, Box::new(|operation, responder| {
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
                OpType::REGISTER => {
                    let mut o = Operation::new();
                    println!("Creating ack");
                    o.set_operation_type(OpType::ACK);

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

    // fn setup_curve(
    //     &self,
    //     s: &mut Socket,
    //     config: &Config
    // ) -> ZmqResult<()> {
    //     // will raise EINVAL if not linked against libsodium
    //     // The ubuntu package is linked so this shouldn't fail
    //     s.set_curve_server(true)?;
    //     let mut pubkey_path = config.config_path.clone();
    //     pubkey_path.push("ecpubkey.pem");
    //     let mut key_path = config.config_path.clone();
    //     key_path.push("ecpubkey.key");
    //     if let Ok(mut file) = File::open(&key_path) {
    //         let mut key = String::new();
    //         let _ = file.read_to_string(&mut key);
    //         s.set_curve_secretkey(&key)?;
    //     } else {
    //         println!("Creating new curve keypair");
    //         let keypair = zmq::CurveKeyPair::new()?;
    //         s.set_curve_secretkey(&keypair.secret_key)?;

    //         let mut f = File::create(pubkey_path).unwrap();
    //         f.write(keypair.public_key.as_bytes()).unwrap();
    //         let mut f = File::create(key_path).unwrap();
    //         f.write(keypair.secret_key.as_bytes()).unwrap();
    //     }

    //     println!("Server mechanism: {:?}", s.get_mechanism());
    //     println!("Curve server: {:?}", s.is_curve_server());

    //     Ok(())
    // }

    // /*
    // Server that manages disks
    // */
    // fn zmq_listen(
    //     &self,
    //     config: &Config,
    // ) -> ZmqResult<()> {
    //     println!("Starting zmq listener with version({:?})", zmq::version());
    //     let context = zmq::Context::new();
    //     let mut responder = context.socket(zmq::REP)?;

    //     println!("Listening on {}", config.zmq_address);
    //     // Fail to start if this fails
    //     self.setup_curve(
    //         &mut responder,
    //         config,
    //     )?;
    //     assert!(
    //         responder
    //             .bind(&config.zmq_address)
    //             .is_ok()
    // );
    //     println!("Going into the zmq loop");
    //     let duration = Duration::from_millis(10);
    //     loop {
    //         match responder.recv_bytes(0) {
    //             Ok(msg) => {
    //                 println!("Got msg len: {}", msg.len());
    //                 println!("Parsing msg {:?} as hex", msg);
    //                 let operation = match parse_from_bytes::<Operation>(&msg) {
    //                     Ok(bytes) => bytes,
    //                     Err(e) => {
    //                         println!("Failed to parse_from_bytes {:?}.  Ignoring request", e);
    //                         continue;
    //                     }
    //                 };
    //                 println!("Operation is: {:?}", operation);
    //                 match operation.get_operation_type() {
    //                     OpType::REGISTER => {
    //                         let mut o = Operation::new();
    //                         println!("Creating ack");
    //                         o.set_operation_type(OpType::ACK);

    //                         let encoded = o.write_to_bytes().unwrap();
    //                         let msg = Message::from_slice(&encoded)?;
    //                         responder.send_msg(msg, 0)?;
    //                     }
    //                     _ => {
    //                         println!("Not quite handling {:?} yet", operation);
    //                     }
    //                 }
    //             },
    //             Err(e) => {
    //                 println!("Failed to recieve bytes: {:?}", e);
    //                 return Err(e);
    //             },
    //         }
    //         //.expect("Failed to recieve bytes?");


    //         thread::sleep(duration);
    //     }
    // }

    fn route(
        &self,
        req: Request,
    ) -> Box<
        Future<
            Item = Response<Box<Stream<Item = Chunk, Error = hyper::Error>>>,
            Error = hyper::Error,
        >,
    > {
        match (req.method(), req.path()) {
            (&Method::Post, _) => {
                let path = req.path().to_string();
                let mut parts = path.split("/").clone();
                let _ = parts.next();
                match (parts.next(), parts.next()) {
                    (Some("webhook"), Some(name)) => webhook(name, req, &self.plugins),
                    (_first, _second) => hello(req),
                }
            }
            _ => {
                let mut response = Response::new();
                println!("Received request: {} {}", req.method(), req.path());
                response.set_status(StatusCode::NotFound);
                Box::new(futures::future::ok(response))
            }
        }
    }

    fn with_plugin_path(mut self, path: PathBuf) -> Self {
        self.plugin_path = Some(path);
        self
    }

    fn load_plugins(mut self) -> Self {
        let path = self.plugin_path.clone();
        if let Some(ref path) = path {
            let mut plugins = vec![];
            if let Ok(dir) = read_dir(path) {
                for file in dir {
                    if let Ok(file) = file {
                        if let Ok(output) = Command::new(file.path()).output() {
                            if let Some(plugin) = PluginEntry::try_from(&output.stdout, file.path())
                            {
                                if !plugins.contains(&plugin) {
                                    plugins.push(plugin);
                                }
                            } else {
                                println!("Had a problem loading pluginn at {:?}", file.path());
                            }
                        }
                    }
                }
            }
            self.plugins = plugins;
        }
        println!("Self is {:?}", self);
        self
    }
}
