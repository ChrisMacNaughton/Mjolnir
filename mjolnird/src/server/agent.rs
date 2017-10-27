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

pub struct Agent {
    masters: Vec<SocketAddr>,
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
    pub fn bind(addr: SocketAddr, masters: Vec<SocketAddr>) -> Result<(), hyper::Error> {
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
        println!("Encoded: {}", encode_hex(&encoded));
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