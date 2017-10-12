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

use protobuf::Message as ProtobufMsg;
use protobuf::core::parse_from_bytes;

use api;
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
    // type Response = Response;
    type Response = Response<Box<Stream<Item=Chunk, Error=Self::Error>>>;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        

        let phrase = "Hello, from Master";
        
        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let mut response = Response::new();
                println!("Received request: {} {}", req.method(), req.path());
                let body: Box<Stream<Item=_, Error=_>> = Box::new(Body::from(phrase));
                response.set_body(body);
                // response.set_body(phrase);
                Box::new(futures::future::ok(response))
            },
            (&Method::Post, "/register") => {
                Box::new(
                    req.body().concat2().map(|body| {
                        let mut response: Response<Box<Stream<Item=Chunk, Error=Self::Error>>> = Response::new();
                        // println!("Body: \n{}", body.wait().unwrap());
                        println!("body: {}", to_hex_string(&body));
                        match parse_from_bytes::<api::agent::Register>(&body) {
                            Ok(agent) => {
                                response.set_status(StatusCode::ImATeapot);
                                // TODO save/update this agent into the database
                                println!("Registered: {:?}", agent);
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
        // do real RPC instead
        let mut agent = api::agent::Register::new();
        agent.set_ip(format!("{}", addr.ip()));
        agent.set_port(addr.port().into());
        println!("Have an agent: {:?}", agent);
        let encoded = agent.write_to_bytes().unwrap();
        println!("Encoded: {}", to_hex_string(&encoded));
        let uri = format!("http://{}/register", master).parse()?;
        let mut req = Request::new( hyper::Method::Post, uri);
        {
            let headers = req.headers_mut();
            headers.set(ContentLength(encoded.len() as u64));
        }
            
        req.set_body(encoded);
        println!("Req is: {:?}", req);
        let work = client.request(req).map(|res| {
            println!("Response: {}", res.status());
        });
        // let work = client.get(uri).map(|res| {
        //     println!("Response: {}", res.status());
        // });
        core.run(work)?;
        // server.handle().spawen(work);
        server.run()
    }
}

fn to_hex_string(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    strs.join(" ")
}