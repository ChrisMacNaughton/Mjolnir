use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures;
// use futures::future::Future;
use futures::{Future, Stream};

use hyper;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

use hyper::{Body, Chunk, Client, Method, StatusCode};

use tokio_core::reactor::Core;

use protobuf::hex::encode_hex;
use protobuf::Message as ProtobufMsg;
use protobuf::core::parse_from_bytes;

use mjolnir_api as api;

#[derive(Clone, Default)]
pub struct Master {
    agents: Arc<Mutex<Vec<SocketAddr>>>
}

impl Service for Master {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    // type Response = Response;
    type Response = Response<Box<Stream<Item=Chunk, Error=Self::Error>>>;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Post, _) => {
                let path = req.path().to_string();
                let mut parts = path.split("/").clone();
                let _ = parts.next();
                match (parts.next(), parts.next()){
                    (Some("webhook"), Some(name)) => {
                        webhook(name, req)
                    }
                    (_first, _second) => {
                        hello(req)
                    }
                }
            },
            (&Method::Get, _) => {
                let path = req.path().to_string();
                let mut parts = path.split("/").clone();
                let _ = parts.next();
                match (parts.next(), parts.next()){
                    (Some("webhook"), Some(name)) => {
                        webhook(name, req)
                    }
                    (_first, _second) => {
                        hello(req)
                    }
                }
            },
            (&Method::Post, "/register") => {
                let agents_arc = self.agents.clone();
                let agent_ip = req.remote_addr().unwrap().ip();
                Box::new(
                    req.body().concat2().map(move |body| {
                        let mut response: Response<Box<Stream<Item=Chunk, Error=Self::Error>>> = Response::new();
                        // println!("Body: \n{}", body.wait().unwrap());
                        println!("body: {}", encode_hex(&body));
                        match parse_from_bytes::<api::agent::Register>(&body) {
                            Ok(mut agent) => {
                                agent.set_ip(format!("{}", agent_ip));
                                let mut agents = agents_arc.lock().unwrap();
                                let addr = SocketAddr::new(agent.get_ip().parse().unwrap(), agent.get_port() as u16);
                                if ! agents.contains(&addr) {
                                    agents.push(addr);
                                }
                                response.set_status(StatusCode::ImATeapot);
                                // TODO save/update this agent into the database
                                println!("Registered: {:?}", agent);
                                println!("We know about {} agents", agents.len());
                            },
                            Err(e) => {
                                println!("Failed to parse_from_bytes {:?}", e);
                                response.set_status(StatusCode::BadRequest);
                            }
                        };
                        response
                    })
                )
            }
            _ => {
                let mut response = Response::new();
                println!("Received request: {} {}", req.method(), req.path());
                response.set_status(StatusCode::NotFound);
                Box::new(futures::future::ok(response))
            },
        }
    }
}

fn hello(req: Request) -> Box<Future<Item=Response<Box<Stream<Item=Chunk, Error=hyper::Error>>>, Error=hyper::Error>> {
    let phrase = "Hello, from Master";
    let mut response = Response::new();
    println!("Received request: {} {}", req.method(), req.path());
    let body: Box<Stream<Item=_, Error=_>> = Box::new(Body::from(phrase));
    response.set_body(body);
    // response.set_body(phrase);
    Box::new(futures::future::ok(response))
}

fn webhook(name: &str, req: Request) -> Box<Future<Item=Response<Box<Stream<Item=Chunk, Error=hyper::Error>>>, Error=hyper::Error>> {
    println!("Responding to webook {} at {}", name, req.path());
    Box::new(
        req.body().concat2().map(move |_body| {
            let mut response: Response<Box<Stream<Item=Chunk, Error=hyper::Error>>> = Response::new();
            let body: Box<Stream<Item=_, Error=_>> = Box::new(Body::from("Ok"));
            response.set_body(body);
            response
        })
    )
}

impl Master {
    pub fn bind(addr: SocketAddr) -> Result<(), hyper::Error> {
        let master = Master::default();
        // OH MY GOD THE PAIN TO KEEP THE RIGHT THING ALIVE
        let closure_master = master.clone();
        let hello = move || Ok(closure_master.clone());

        let server = Http::new().bind(&addr, hello)?;
        server.run()
    }
}
