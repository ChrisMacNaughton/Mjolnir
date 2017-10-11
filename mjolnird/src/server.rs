use std::io::{self, Write};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures;
// use futures::future::Future;
use futures::{Future, Stream};

use hyper;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

use hyper::{Client, Method, StatusCode};

use tokio_core::reactor::Core;


use config::{Config, Mode};

#[derive(Default)]
pub struct Master {
    agents: Arc<Mutex<Vec<SocketAddr>>>
}

pub struct Agent {
    masters: Vec<SocketAddr>,
}

// const PHRASE: &'static str = "Hello, World!";

pub fn bind(config: &Config) -> Result<(), hyper::Error> {
    match config.mode {
        Mode::Agent(ref masters) => Agent::bind(config.bind_address.clone(), masters.clone()),
        Mode::Master => Master::bind(config.bind_address.clone()),
    }
}

impl Service for Master {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        let mut response = Response::new();

        let phrase = "Hello, from Master";
        
        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                println!("Received request: {} {}", req.method(), req.path());
                response.set_body(phrase);
            },
            (&Method::Get, "/register") => {
                // we'll be back
                println!("Received register request: {} {:?}", req.method(), req);
                let agent_port = req.query().unwrap().split('=').last().unwrap().split(':').last().unwrap();
                let agent_ip = req.remote_addr().unwrap().ip();
                let agent_addr = format!("{}:{}", agent_ip, agent_port).parse().unwrap();
                {
                    let mut agents = self.agents.lock().unwrap();
                    if ! agents.contains(&agent_addr) {
                        agents.push(agent_addr);
                    }
                    
                    println!("We know about {} agents", agents.len());
                }
            },
            _ => {
                println!("Received request: {} {}", req.method(), req.path());
                response.set_status(StatusCode::NotFound);
            },
        };

        Box::new(futures::future::ok(response))
    }
}

impl Master {
    fn bind(addr: SocketAddr) -> Result<(), hyper::Error> {
        let hello = || Ok(Master::default());

        let server = Http::new().bind(&addr, hello)?;
        server.run()
    }
}

impl Service for Agent {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;
    //alternately:
    // type Future = futures::future::FutureResult<Self::Response, Self::Error>;


    fn call(&self, _req: Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        let phrase = "Hello, from Agent";
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(phrase.len() as u64))
                .with_body(phrase)
        ))
    }
}

impl Agent {
    fn bind(addr: SocketAddr, masters: Vec<SocketAddr>) -> Result<(), hyper::Error> {
        let master = masters[0].clone();
        let server = Http::new().bind(&addr, move || Ok(Agent {masters: masters.clone()}))?;

        let mut core = Core::new()?;
        let client = Client::new(&core.handle());
        let uri = format!("http://{}/register?my_ip={}", master, addr).parse()?;
        let work = client.get(uri).map(|res| {
            println!("Response: {}", res.status());
        });
        core.run(work)?;
        // server.handle().spawen(work);
        server.run()
    }
}