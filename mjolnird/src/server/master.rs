use std::time::{Instant, Duration};
use std::fs::{File, read_dir};
use std::io::{self, Read};
use std::net::IpAddr;
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

use zmq::{Message, Result as ZmqResult};

use protobuf::Message as ProtobufMsg;

use mjolnir::Pipeline;
use mjolnir_api::{Alert, Operation, OperationType as OpType, PluginEntry, Register, RemediationResult, Remediation};
use server::{zmq_listen, connect, server_pubkey, load_pipeline};
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

    #[test]
    fn it_validates_pipelines() {
        let args = Config::matches().get_matches_from(vec![
            "mjolnird",
            "--bind=192.168.0.101:11011",
            "--config=../examples/configs",
            // "--plugins=/usr/local/share",
            "--ip=127.0.0.1",
            "master",
        ]);
        let config = Config::from_args(args);
        let (mut master, _receiver) = Master::new();
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
            .load_pipelines(&config);
        assert!(master.pipelines.len() == 1);
    }
}

#[derive(Clone, Debug)]
struct Agent {
    ip: IpAddr,
    hostname: String,
    port: u16,
    last_seen: Instant,
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
    pub fn remediate(&self, alert: Alert, remediation: &Remediation, config: &Config) {
        let server_pubkey = server_pubkey(&config);
         match connect(&self.ip.to_string(), self.port, &server_pubkey){
            Ok(socket) => {
                let mut o = Operation::new();
                // println!("Creating PING");
                o.set_operation_type(OpType::REMEDIATE);
                let mut remediation = remediation.clone();
                remediation.target = alert.source;
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
    println!("About to run command: {:?}", cmd);
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
        println!("Responding to webook {} at {}", name, req.path());
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
            .load_pipelines(&config);

        let http_config = config.clone();

        // OH MY GOD THE PAIN TO KEEP THE RIGHT THING ALIVE
        let closure_master = master.clone();
        thread::spawn(move || {
            let master_server = move || Ok(closure_master.clone());
            let server = Http::new().bind(&http_config.bind_address, master_server)?;
            server.run()
        });
        let background_agents = master.agents.clone();
        let background_config = config.clone();
        let ping_duration = Duration::from_millis(500);
        let mpsc_duration = Duration::from_millis(50);
        thread::spawn(move|| {
            let server_pubkey = server_pubkey(&background_config);
            loop {
                if let Ok(agents) = background_agents.try_lock() {
                    for agent in agents.iter() {
                        match connect(&agent.hostname, agent.port, &server_pubkey){
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
        let background_config = config.clone();
        thread::spawn(move|| {
            loop {
                match receiver.try_recv() {
                    Ok(s) => {
                        match s {
                            MasterAction::Webhook(s) => bg_master.handle_webhook(s),
                            MasterAction::Alert(alert) => bg_master.remediate(alert, &background_config)
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

    fn remediate(&self, alert: Alert, config: &Config) {
        if let Some(pipeline) = self.pipelines.iter().find(|p| p.trigger == alert) {
            println!("Remediating {:?}", alert);
            if let Some(source) = alert.source.clone() {
                if let Ok(agents) = self.agents.try_lock() {
                    if let Some(agent) = agents.iter().find(|a| a.hostname == source || a.ip.to_string() == source).clone() {
                        println!("Have an agent: {:?}", agent);
                        agent.remediate(alert, &pipeline.action, config);
                    }
                }
            }
        } else {
            println!("Ignoring {:?}, no configured pipeline", alert);
        }
    }

    fn setup_zmq(&self, config: &Config) -> ZmqResult<()> {
        let agents: Arc<Mutex<Vec<Agent>>> = self.agents.clone();
        let sender = self.sender.clone();
        zmq_listen(
            config,
            Box::new(move |operation, responder| {
                // let agents_arc = agents.clone();
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
                        let mut o = Operation::new();
                        println!("Creating ack");
                        o.set_operation_type(OpType::ACK);

                        let encoded = o.write_to_bytes().unwrap();
                        let msg = Message::from_slice(&encoded)?;
                        responder.send_msg(msg, 0)?;
                        let mut agents = agents.lock().unwrap();
                        let register: Register = operation.get_register().clone().into();
                        let agent = Agent {
                            ip: register.ip,
                            hostname: register.hostname.clone(),
                            port: register.port,
                            last_seen: Instant::now(),
                        };
                        let mut updated = false;
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
                    OpType::ALERT => {
                        let mut o = Operation::new();
                        println!("Creating ack for alert");
                        o.set_operation_type(OpType::ACK);

                        let encoded = o.write_to_bytes().unwrap();
                        let msg = Message::from_slice(&encoded)?;
                        responder.send_msg(msg, 0)?;

                        let alert: Alert = operation.get_alert().into();
                        let _ = sender.send(MasterAction::Alert(alert));
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
                            if let Ok(plugin) = PluginEntry::try_from(
                                &output.stdout,
                                file.path(),
                            )
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
        self
    }

    fn load_pipelines(mut self, config: &Config) -> Self {
        let pipelines = load_pipeline(config);

        match self.validate_pipelines(&pipelines) {
            Ok(()) => {},
            Err(e) => panic!("Couldn't load plugin that matches your pipeline: {:?}", e),
        }

        self.pipelines = pipelines;
        self
    }

    fn validate_pipelines(&self, pipelines: &Vec<Pipeline>) -> Result<(), String> {
        for pipeline in pipelines {
            println!("Validating we have a plugin configured for '{}'", pipeline.action.plugin);
            if !self.plugins.iter().any(|p| p.name == pipeline.action.plugin) {
                return Err(format!("{} has no matching plugin", pipeline.action.plugin));
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