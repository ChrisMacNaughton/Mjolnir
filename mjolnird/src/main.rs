#[macro_use] extern crate clap;
extern crate hyper;
extern crate service_fn;
extern crate mjolnir;

use std::net::SocketAddr;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Response};
use service_fn::service_fn;

mod config;

use config::{Config, Mode};

fn main() {
    println!("Welcome to Mjölnir");
    
    let config = Config::get_config();
    println!("About to start with {:?}", config);

    let _ = bind(config.bind_address);
}

fn bind(addr: SocketAddr) -> Result<(), hyper::Error> {
    let hello = || Ok(service_fn(|_req|{
        Ok(Response::<hyper::Body>::new()
            .with_header(ContentLength(5))
            .with_header(ContentType::plaintext())
            .with_body("Hello"))
    }));

    let server = Http::new().bind(&addr, hello)?;
    server.run()
}