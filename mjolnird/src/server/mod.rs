// use hyper::Error;
use std::thread;
use std::time::Duration;
use std::fs::{read_dir, File};
use std::io::{Read, Write};

use protobuf::Message as ProtobufMsg;
use zmq::{self, Message, Socket, Result as ZmqResult};

use config::{Config, Mode};

mod master;
mod agent;


use mjolnir_api::{Operation, OperationType as OpType, parse_from_bytes};

// const PHRASE: &'static str = "Hello, World!";

pub fn bind(config: Config) -> ZmqResult<()>{
    match config.mode.clone() {
        Mode::Agent(masters) => agent::Agent::bind(config, masters),
        Mode::Master => master::Master::bind(config),
    }
}

fn connect(host: &str, port: u16, server_publickey: &str) -> ZmqResult<Socket> {
    println!("Starting zmq sender with version({:?})", zmq::version());
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ)?;
    let client_keypair = zmq::CurveKeyPair::new()?;

    requester.set_curve_serverkey(server_publickey)?;
    requester.set_curve_publickey(&client_keypair.public_key)?;
    requester.set_curve_secretkey(&client_keypair.secret_key)?;
    println!("Connecting to tcp://{}:{}", host, port);
    assert!(
        requester
            .connect(&format!("tcp://{}:{}", host, port))
            .is_ok()
    );
    println!("Client mechanism: {:?}", requester.get_mechanism());

    Ok(requester)
}

fn setup_curve(
    s: &mut Socket,
    config: &Config
) -> ZmqResult<()> {
    // will raise EINVAL if not linked against libsodium
    // The ubuntu package is linked so this shouldn't fail
    s.set_curve_server(true)?;
    let mut pubkey_path = config.config_path.clone();
    pubkey_path.push("ecpubkey.pem");
    let mut key_path = config.config_path.clone();
    key_path.push("ecpubkey.key");
    if let Ok(mut file) = File::open(&key_path) {
        let mut key = String::new();
        let _ = file.read_to_string(&mut key);
        s.set_curve_secretkey(&key)?;
    } else {
        println!("Creating new curve keypair");
        let keypair = zmq::CurveKeyPair::new()?;
        s.set_curve_secretkey(&keypair.secret_key)?;

        let mut f = File::create(pubkey_path).unwrap();
        f.write(keypair.public_key.as_bytes()).unwrap();
        let mut f = File::create(key_path).unwrap();
        f.write(keypair.secret_key.as_bytes()).unwrap();
    }

    println!("Server mechanism: {:?}", s.get_mechanism());
    println!("Curve server: {:?}", s.is_curve_server());

    Ok(())
}

/*
Server that manages disks
*/
fn zmq_listen(
    config: &Config,
    callback: Box<Fn(Operation, &Socket) -> ZmqResult<()>>
) -> ZmqResult<()> {
    println!("Starting zmq listener with version({:?})", zmq::version());
    let context = zmq::Context::new();
    let mut responder = context.socket(zmq::REP)?;

    println!("Listening on {}", config.zmq_address);
    // Fail to start if this fails
    setup_curve(
        &mut responder,
        config,
    )?;
    assert!(
        responder
            .bind(&config.zmq_address)
            .is_ok()
);
    println!("Going into the zmq loop");
    let duration = Duration::from_millis(10);
    loop {
        match responder.recv_bytes(0) {
            Ok(msg) => {
                println!("Got msg len: {}", msg.len());
                println!("Parsing msg {:?} as hex", msg);
                let operation = match parse_from_bytes::<Operation>(&msg) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        println!("Failed to parse_from_bytes {:?}.  Ignoring request", e);
                        continue;
                    }
                };
                println!("Operation is: {:?}", operation);
                callback(operation, &responder)?
            },
            Err(e) => {
                println!("Failed to recieve bytes: {:?}", e);
                return Err(e);
            },
        }
        //.expect("Failed to recieve bytes?");


        thread::sleep(duration);
    }
}