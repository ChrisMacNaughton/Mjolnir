use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};

use futures;
// use futures::future::Future;
use futures::{Future, Stream};

use hyper;
// use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

use hyper::{Body, Chunk, Method, StatusCode};

use protobuf::hex::encode_hex;
// use protobuf::Message as ProtobufMsg;
use protobuf::core::parse_from_bytes;

use mjolnir_api as api;
use mjolnir::PluginEntry;
 
use config::Config;

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
        match (req.method(), req.path()) {
            (&Method::Post, "/register") => {
                let agents_arc = self.agents.clone();
                let agent_ip = req.remote_addr().unwrap().ip();
                Box::new(req.body().concat2().map(move |body| {
                    let mut response: Response<
                        Box<Stream<Item = Chunk, Error = Self::Error>>,
                    > = Response::new();
                    // println!("Body: \n{}", body.wait().unwrap());
                    println!("body: {}", encode_hex(&body));
                    match parse_from_bytes::<api::agent::Register>(&body) {
                        Ok(mut agent) => {
                            agent.set_ip(format!("{}", agent_ip));
                            let mut agents = agents_arc.lock().unwrap();
                            let addr = SocketAddr::new(
                                agent.get_ip().parse().unwrap(),
                                agent.get_port() as u16,
                            );
                            if !agents.contains(&addr) {
                                agents.push(addr);
                            }
                            response.set_status(StatusCode::ImATeapot);
                            // TODO save/update this agent into the database
                            println!("Registered: {:?}", agent);
                            println!("We know about {} agents", agents.len());
                        }
                        Err(e) => {
                            println!("Failed to parse_from_bytes {:?}", e);
                            response.set_status(StatusCode::BadRequest);
                        }
                    };
                    response
                }))
            }
            (&Method::Post, _) => {
                let path = req.path().to_string();
                let mut parts = path.split("/").clone();
                let _ = parts.next();
                match (parts.next(), parts.next()) {
                    (Some("webhook"), Some(name)) => webhook(name, req),
                    (_first, _second) => hello(req),
                }
            }
            (&Method::Get, _) => {
                let path = req.path().to_string();
                let mut parts = path.split("/").clone();
                let _ = parts.next();
                match (parts.next(), parts.next()) {
                    (Some("webhook"), Some(name)) => webhook(name, req),
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
) -> Box<
    Future<Item = Response<Box<Stream<Item = Chunk, Error = hyper::Error>>>, Error = hyper::Error>,
> {
    println!("Responding to webook {} at {}", name, req.path());
    Box::new(req.body().concat2().map(move |_body| {
        let mut response: Response<Box<Stream<Item = Chunk, Error = hyper::Error>>> =
            Response::new();
        let body: Box<Stream<Item = _, Error = _>> = Box::new(Body::from("Ok"));
        response.set_body(body);
        response
    }))
}

impl Master {
    pub fn bind(config: Config) -> Result<(), hyper::Error> {
    
        let master = Master::default().with_plugin_path(config.plugin_path).load_plugins();
        // OH MY GOD THE PAIN TO KEEP THE RIGHT THING ALIVE
        let closure_master = master.clone();
        let master_server = move || Ok(closure_master.clone());

        let server = Http::new().bind(&config.bind_address, master_server)?;
        server.run()
    }

    fn with_plugin_path(mut self, path: Option<PathBuf>) -> Self {
        self.plugin_path = path;
        self
    }

    fn load_plugins(mut self) -> Self {
        let path = self.plugin_path.clone();
        if let Some(ref path) = path {
            let mut plugins = vec![];
            if let Ok(dir) = fs::read_dir(path) {
                for file in dir {
                    if let Ok(file) = file {
                        println!("Trying to load plugin at: {:?}", file.path());
                        if let Ok(output) = Command::new(file.path())
                            .output(){
                            if let Some(plugin) = PluginEntry::try_from(&output.stdout) {
                                if !plugins.contains(&plugin) {
                                    println!("Plugin is: {:?}", plugin);
                                    plugins.push(plugin);
                                } else {
                                    println!("Tried loading a duplicate pluigin named: {}", plugin.name);
                                }
                            } else {
                                println!("Had a problem loading pluginn at {:?}", file.path());
                            }
                            // println!("Output is: {:?}", String::from_utf8_lossy(&output.stdout));
                            
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
