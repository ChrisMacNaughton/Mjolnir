use std::time::{Instant, Duration};
use std::fs::{File, read_dir};
use std::io::{self, Read};
use std::net::{IpAddr, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;

use futures;
// use futures::future::Future;
use futures::{Future, Stream};

use hyper;
use hyper::server::{Http, Request, Response, Service};

use hyper::{Body, Chunk, Method, StatusCode};
use hyper::header::ContentLength;

use zmq::{Message, Result as ZmqResult, Socket};

use protobuf::Message as ProtobufMsg;

use mjolnir::Pipeline;
use mjolnir_api::{Alert, Operation, OperationType as OpType, PluginEntry, Register, RemediationResult, Remediation};
use server::{zmq_listen, connect};
use config::Config;

#[cfg(test)]
mod tests {
    use super::*;
    use mjolnir_api::plugin;

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

    #[test]
    fn it_validates_pipelines() {
        let args = Config::matches().get_matches_from(vec![
            "mjolnird",
            "--config=../examples/configs/mjolnir.toml",
            "master",
        ]);
        let config = Config::from_args(args);
        let (mut master, _receiver) = Master::new(config.clone());
        master = master.with_plugin_path(config.plugin_path.clone());
        master.plugins.push(PluginEntry {
            name: "clean_disk".into(),
            author: "test author".into(),
            version: "test version".into(),
            webhook: true,
            alerts: vec![],
            remediations: vec![],
            path: PathBuf::from("/bin/echo"),
        });
        master = master
            .load_pipelines();
        assert!(master.pipelines.len() == 1);
    }

    #[test]
    fn it_compares_agents() {
        let a1 = Agent {
            ip: "::".parse().unwrap(),
            hostname: "test".into(),
            port: 8080,
            last_seen: Instant::now(),
            public_key: "pub_key".into()
        };

        let a2 = Agent {
            ip: "::".parse().unwrap(),
            hostname: "test".into(),
            port: 8080,
            last_seen: Instant::now() + Duration::from_secs(100),
            public_key: "pub_key".into()
        };

        assert_eq!(a1.clone(), a2);

        let a3 = Agent {
            ip: "127.0.0.1".parse().unwrap(),
            hostname: "test".into(),
            port: 8080,
            last_seen: Instant::now(),
            public_key: "pub_key".into()
        };

        assert_ne!(a1, a3);
    }

    #[test]
    fn it_messages_self() {
        let args = Config::matches().get_matches_from(vec![
            "mjolnird",
            "--config=../examples/configs/empty.toml",
            "master",
        ]);
        let config = Config::from_args(args);
        let (mut master, receiver) = Master::new(config);


        master = master.load_plugins()
            .load_pipelines();

        let result = RemediationResult {
            result: Ok(()),
            alerts: vec![
                Alert {
                    alert_type: "Test".into(),
                    name: Some("placeholder".into()),
                    source: Some("test".into()),
                    args: vec!["testarg=value".into()],
                    next_remediation: 0,
                }],
        };

        let plugin_result: plugin::RemediationResult = result.clone().into();

        let bytes: String = String::from_utf8_lossy(&plugin_result.write_to_bytes().unwrap()).into_owned();

        master.handle_webhook(bytes);

        let action = receiver.try_recv().unwrap();
        match action {
            MasterAction::Alert(alert) => {
                assert_eq!(alert, Alert {
                    alert_type: "Test".into(),
                    name: Some("placeholder".into()),
                    source: Some("test".into()),
                    args: vec!["testarg=value".into()],
                    next_remediation: 0,
                });
            },
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Debug)]
struct Agent {
    ip: IpAddr,
    hostname: String,
    port: u16,
    last_seen: Instant,
    public_key: String,
}

impl PartialEq for Agent {
    fn eq(&self, other: &Agent) -> bool {
        self.ip == other.ip && self.hostname == other.hostname && self.port == other.port
    }
}

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct Remediation {
//     pub plugin: String,
//     pub target: Option<String>,
//     pub args: Vec<String>,
// }
impl Agent {
    pub fn remediate(&self, alert: Alert, remediation: &Remediation) {
        match connect(&self.ip.to_string(), self.port, &self.public_key){
            Ok(socket) => {
                let mut o = Operation::new();
                // println!("Creating PING");
                o.set_operation_type(OpType::REMEDIATE);
                let mut remediation = remediation.clone();
                remediation.target = alert.source.clone();
                remediation.alert = Some(alert);
                o.set_remediate(remediation.into());
                let encoded = o.write_to_bytes().unwrap();
                let msg = Message::from_slice(&encoded).unwrap();
                match socket.send_msg(msg, 0) {
                    Ok(_s) => {},
                    Err(e) => println!("Problem sending remediation request: {:?}", e)
                }
            }
            Err(e) => println!("problem connecting to socket: {:?}", e),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Master {
    agents: Arc<Mutex<Vec<Agent>>>,
    plugins: Vec<PluginEntry>,
    plugin_path: Option<PathBuf>,
    pipelines: Vec<Pipeline>,
    sender: Sender<MasterAction>,
    config: Config,
}

enum MasterAction {
    Webhook(String),
    Alert(Alert),
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

fn process_webhook(hook: PluginEntry, body: String) -> String {
    // println!("Hook is: {:?}", hook);
    let mut cmd = Command::new(hook.path);
    cmd.arg(format!("plugin={}", hook.name));
    cmd.arg(format!("body={}", body));
    // println!("About to run command: {:?}", cmd);
    match cmd.output() {
        Ok(output) => {
            match String::from_utf8(output.stdout) {
                Ok(s) => s,
                Err(e) => format!("{:?}", e),
            }
        }
        Err(e) => format!("{:?}", e)
    }
}

impl Master {
    fn new(config: Config) -> (Master, Receiver<MasterAction>) {
        let (sender, receiver) = channel();
        (
            Master {
                agents: Arc::new(Mutex::new(vec![])),
                plugins: vec![],
                plugin_path: None,
                pipelines: vec![],
                sender: sender,
                config: config
            },
            receiver
        )
    }

    fn handle_webhook(&self, data: String) {
        // println!("About to parse {}", data);
        let result = RemediationResult::from_string(&data);
        for alert in result.alerts {
            let _ = self.sender.send(MasterAction::Alert(alert));
        }
    }

    fn webhook(
        &self,
        name: &str,
        req: Request,
    ) -> Box<
        Future<
            Item = Response<Box<Stream<Item = Chunk, Error = hyper::Error>>>,
            Error = hyper::Error,
        >,
    > {
        // println!("Responding to webook {} at {}", name, req.path());
        // let plugins = plugins.clone();
        let hook = self.plugins
            .iter()
            .filter(|wh| wh.webhook)
            .filter(|wh| wh.name == name)
            .nth(0)
            .map(|p| p.clone());
        // let hook: Option<PluginEntry> = *hook.clone();
        let sender = self.sender.clone();
        Box::new(req.body().concat2().map(move |body| {
            // let plugins = plugins.clone();
            let body: Box<Stream<Item = _, Error = _>> = if let Some(hook) = hook {
                match String::from_utf8(body.to_vec()) {
                    Ok(s) => {
                        let webhook_output = process_webhook(hook, s);
                        let _ = sender.send(MasterAction::Webhook(webhook_output));
                        Box::new(Body::from("Ok"))
                    },
                    Err(_) => Box::new(Body::from("Invalid Body")),
                }
            } else {
                Box::new(Body::from("Unknown Webhook"))
            };
            let mut response: Response<Box<Stream<Item = Chunk, Error = hyper::Error>>> =
                Response::new();

            response.set_body(body);
            response
        }))
    }

    pub fn bind(config: Config) -> ZmqResult<()> {
        let (mut master, receiver) = Master::new(config.clone());
        master = master.with_plugin_path(config.plugin_path.clone())
            .load_plugins()
            .load_pipelines();

        let http_config = config.clone();

        // OH MY GOD THE PAIN TO KEEP THE RIGHT THING ALIVE
        let closure_master = master.clone();
        thread::spawn(move || {
            let master_server = move || Ok(closure_master.clone());
            let server = Http::new().bind(&(http_config.bind_ip, http_config.http_port).to_socket_addrs().unwrap().next().unwrap(), master_server)?;
            server.run()
        });
        let background_agents = master.agents.clone();
        let ping_duration = Duration::from_millis(500);
        let mpsc_duration = Duration::from_millis(50);
        thread::spawn(move|| {
            loop {
                if let Ok(agents) = background_agents.try_lock() {
                    for agent in agents.iter() {
                        match connect(&agent.hostname, agent.port, &agent.public_key){
                            Ok(socket) => {
                                let mut o = Operation::new();
                                // println!("Creating PING");
                                o.set_operation_type(OpType::PING);

                                let encoded = o.write_to_bytes().unwrap();
                                let msg = Message::from_slice(&encoded).unwrap();
                                match socket.send_msg(msg, 0) {
                                    Ok(_s) => {},
                                    Err(e) => println!("Problem sending ping: {:?}", e)
                                }
                            }
                            Err(e) => println!("problem connecting to socket: {:?}", e),
                        }
                    }
                } else {
                    println!("Failed to lock agents mutex for PING");
                }
            
                thread::sleep(ping_duration);
            }
        });
        let bg_master = master.clone();
        thread::spawn(move|| {
            loop {
                match receiver.try_recv() {
                    Ok(s) => {
                        match s {
                            MasterAction::Webhook(s) => bg_master.handle_webhook(s),
                            MasterAction::Alert(alert) => bg_master.remediate(alert)
                        }
                    },
                    Err(_e) => {},
                }
                thread::sleep(mpsc_duration);
            }
        });
        let _ = master.setup_zmq(&config)?;
        thread::park();
        Ok(())
    }

    fn remediate(&self, alert: Alert) {
        if let Some(pipeline) = self.pipelines.iter().find(|p| p.trigger == alert) {
            println!("Remediating {:?}", alert);
            if let Some(source) = alert.source.clone() {
                if let Ok(agents) = self.agents.try_lock() {
                    if let Some(agent) = agents.iter().find(|a| a.hostname == source || a.ip.to_string() == source).clone() {
                        println!("Have an agent: {:?}", agent);
                        if let Some(ref action) = pipeline.actions.get(alert.next_remediation as usize) {
                            agent.remediate(alert, action);
                        } else {
                            println!("Pipeline for {} is exhausted, please intervene manually", alert.alert_type);
                        }
                    }
                } else {
                    println!("Failed to get lock");
                }
            } else {
                println!("PENDING : Handle alerts with no target");
            }
        } else {
            println!("Ignoring {:?}, no configured pipeline", alert);
        }
    }

    fn setup_zmq(&self, config: &Config) -> ZmqResult<()> {
        // let agents: Arc<Mutex<Vec<Agent>>> = self.agents.clone();
        let agents = self.agents.clone();
        let sender = self.sender.clone();
        let boxed_config = config.clone();
        zmq_listen(
            config,
            Box::new(move |operation, responder| {
                let agents = agents.clone();
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
                    OpType::REGISTER => {
                        let register: Register = operation.get_register().into();
                        if register.secret != boxed_config.secret {
                            println!("Bad agent registration: {:?}", register);
                            let mut o = Operation::new();
                            o.set_operation_type(OpType::NACK);
                            let encoded = o.write_to_bytes().unwrap();
                            let msg = Message::from_slice(&encoded)?;
                            responder.send_msg(msg, 0)?;
                        } else {
                            ack(responder)?;
                            let agent = Agent {
                                ip: register.ip,
                                hostname: register.hostname.clone(),
                                port: register.port,
                                last_seen: Instant::now(),
                                public_key: register.public_key,
                            };
                            let mut updated = false;
                            {
                                let mut agents = agents.lock().expect("Couldn't lock agents");
                                {
                                    let known = agents.iter_mut().filter(|a| **a == agent).nth(0);
                                    if let Some(known_agent) = known {
                                        known_agent.last_seen = agent.last_seen;
                                        updated = true;
                                    }
                                }
                                if !updated {
                                    println!("Adding a new agent: {:?}!", agent);
                                    agents.push(agent);
                                }

                                println!("#{} Agents", agents.len());
                            }
                        }
                    }
                    OpType::ALERT => {
                        ack(responder)?;

                        let alert: Alert = operation.get_alert().into();
                        let _ = sender.send(MasterAction::Alert(alert));
                    }
                    OpType::REMEDIATION_RESULT => {
                        ack(responder)?;

                        let result: RemediationResult = operation.get_result().into();
                        if result.result.is_err() {
                            // let mut action = result.alert
                            for alert in result.alerts {
                                let _ = sender.send(MasterAction::Alert(alert));
                            }
                        }
                        
                    }
                    _ => {
                        println!("Not quite handling {:?} yet", operation);
                        ack(responder)?
                    }
                }
                Ok(())
            }),
        )
    }

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
            (&Method::Get, "/pubkey.pem") => {
                let mut path = self.config.key_path.clone();
                path.push("ecpubkey.pem");
                read_file(&path)
            }
            (&Method::Get, _) => {
                let path = req.path().to_string();
                let mut parts = path.split("/").clone();
                let _ = parts.next();
                match (parts.next(), parts.next()) {
                    (Some("plugin"), Some(name)) => {
                        if let Some(local_path) = local_path_for_request(&format!("/{}", name), &self.config.plugin_path) {
                            read_file(&local_path)
                        } else {
                            not_found(&req)
                        }
                    },
                    (_first, _second) => {
                        not_found(&req)
                    },
                }
            }
            (&Method::Post, _) => {
                let path = req.path().to_string();
                let mut parts = path.split("/").clone();
                let _ = parts.next();
                match (parts.next(), parts.next()) {
                    (Some("webhook"), Some(name)) => self.webhook(name, req),
                    (_first, _second) => {
                        not_found(&req)
                    },
                }
            }
            _ => {
                not_found(&req)
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
                            match PluginEntry::try_from(
                                &output.stdout,
                                &file.path(),
                            ) {
                                Ok(plugin) => {
                                    if !plugins.contains(&plugin) {
                                        plugins.push(plugin);
                                    }
                                }
                                Err(e) => println!("Had a problem loading plugin at {}: {:?}", file.path().display(), e)
                            }
                        }
                    }
                }
            }
            self.plugins = plugins;
        }
        self
    }

    fn load_pipelines(mut self) -> Self {
        self.pipelines = {
            let pipelines = &self.config.pipelines;

            match self.validate_pipelines(&pipelines) {
                Ok(()) => {},
                Err(e) => panic!("Couldn't load plugin that matches your pipeline: {:?}", e),
            }

            self.config.pipelines.clone()
        };
        self
    }

    fn validate_pipelines(&self, pipelines: &Vec<Pipeline>) -> Result<(), String> {
        for pipeline in pipelines {
            for action in &pipeline.actions {
                println!("Validating we have a plugin configured for '{}'", action.plugin);
                if !self.plugins.iter().any(|p| p.name == action.plugin) {
                    return Err(format!("{} has no matching plugin", action.plugin));
                }
            }
        }
        Ok(())
    }
}

fn local_path_for_request(request_path: &str, root_dir: &Path) -> Option<PathBuf> {
    // This is equivalent to checking for hyper::RequestUri::AbsoluteUri
    if !request_path.starts_with("/") {
        return None;
    }
    // Trim off the url parameters starting with '?'
    let end = request_path.find('?').unwrap_or(request_path.len());
    let request_path = &request_path[0..end];

    // Append the requested path to the root directory
    let mut path = root_dir.to_owned();
    if request_path.starts_with('/') {
        path.push(&request_path[1..]);
    } else {
        return None;
    }

    // Maybe turn directory requests into index.html requests
    if request_path.ends_with('/') {
        path.push("index.html");
    }

    Some(path)
}

fn read_file(path: &Path) -> Box<
    Future<
        Item = Response<Box<Stream<Item = Chunk, Error = hyper::Error>>>,
        Error = hyper::Error,
    >> {
    match File::open(&path) {
        Ok(mut file) => {
            let mut buf = Vec::new();
            match file.read_to_end(&mut buf) {
                Ok(_) => {
                    let len = buf.len();
                    let body: Box<Stream<Item = _, Error = _>> = Box::new(Body::from(buf));
                    // response.set_body(body);
                    let response = Response::new()
                        .with_status(StatusCode::Ok)
                        .with_header(ContentLength(len as u64))
                        .with_body(body);
                    Box::new(futures::future::ok(response))
                }
                Err(_) => internal_server_error(),
            }
        }
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    Box::new(futures::future::ok(Response::new()
                        .with_status(StatusCode::NotFound)))
                },
                _ => internal_server_error(),
            }
        }
    }
}

fn internal_server_error() -> Box<
    Future<
        Item = Response<Box<Stream<Item = Chunk, Error = hyper::Error>>>,
        Error = hyper::Error,
    >> {
    Box::new(futures::future::ok(Response::new()
        .with_status(StatusCode::InternalServerError)
        .with_header(ContentLength(0))))
}

fn not_found(req: &Request) -> Box<
    Future<
        Item = Response<Box<Stream<Item = Chunk, Error = hyper::Error>>>,
        Error = hyper::Error,
    >> {
        println!("Received request: {} {}", req.method(), req.path());;
        Box::new(futures::future::ok(Response::new()
            .with_status(StatusCode::NotFound)
            .with_header(ContentLength(0))))
}

fn ack(responder: &Socket) -> ZmqResult<()>{
    let mut o = Operation::new();
    // println!("Creating ack for alert");
    o.set_operation_type(OpType::ACK);

    let encoded = o.write_to_bytes().unwrap();
    let msg = Message::from_slice(&encoded)?;
    responder.send_msg(msg, 0)
}