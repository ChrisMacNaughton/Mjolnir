use std::net::SocketAddr;
// use std::sync::{Arc, Mutex};

// use futures;
// use futures::future::Future;
// use futures::Future;

use hyper;
// use hyper::header::ContentLength;
// use hyper::server::{Http, Request, Response, Service};

// use hyper::Client;

// use tokio_core::reactor::Core;

// use protobuf::hex::encode_hex;
// use protobuf::Message as ProtobufMsg;
// use protobuf::core::parse_from_bytes;

// use mjolnir_api as api;

#[derive(Clone)]
pub struct Agent {
    masters: Vec<SocketAddr>,
}

impl Agent {
    pub fn bind(_addr: SocketAddr, masters: Vec<SocketAddr>) -> Result<(), hyper::Error> {
        let _agent = Agent {
                masters: masters,
            };
        // do some stuff with zmq here 
        Ok(())
    }
}
