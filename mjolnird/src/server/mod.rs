// use hyper::Error;
use std::thread;
use std::time::Duration;
use std::fs::{File, read_dir};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, RwLock};

use base64::encode;
use reqwest;
use zmq::{self, Message, Socket, Result as ZmqResult};

use config::{Config, Master, Mode};

mod master;
mod agent;

use protobuf::Message as ProtobufMsg;

use mjolnir_api::{Operation, parse_from_bytes, PluginEntry, Remediation, RemediationResult, OperationType as OpType};

use mjolnir::Pipeline;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, remove_dir_all};
    use std::path::PathBuf;

    #[test]
    fn it_sets_up_crypto() {
        let key_path = PathBuf::from("/tmp/mjolnir");

        let _ = remove_dir_all(&key_path);
        let _ = create_dir_all(&key_path);
        let args = Config::matches().get_matches_from(vec![
            "mjolnird",
            "--config=../examples/configs/mjolnir.toml",
            "master",
        ]);
        let mut config = Config::from_args(args);
        config.key_path = key_path;

        let context = zmq::Context::new();
        let mut responder = context.socket(zmq::REP).unwrap();

        let r = setup_curve(&mut responder, &config);
        assert!(r.is_ok());

        let key = server_pubkey(&config);

        let context = zmq::Context::new();
        let mut responder = context.socket(zmq::REP).unwrap();

        let r = setup_curve(&mut responder, &config);
        assert!(r.is_ok());

        assert_eq!(key, server_pubkey(&config));
    }

}

pub fn reload<T: AsRef<str>>(master: &Master, pubkey: T) {
    match connect(&master.ip, master.zmq_port, pubkey.as_ref()){
        Ok(socket) => {
            let mut o = Operation::new();
            println!("Asking {} to reload", master.ip);
            o.set_operation_type(OpType::RELOAD);

            let encoded = o.write_to_bytes().unwrap();
            let msg = Message::from_slice(&encoded).unwrap();
            match socket.send_msg(msg, 0) {
                Ok(_s) => {},
                Err(e) => println!("Problem sending reload: {:?}", e)
            }
        }
        Err(e) => println!("problem connecting to socket: {:?}", e),
    }
    println!("Done!");
}

pub fn bind(config: Config) -> ZmqResult<()> {
    match &config.mode {
        &Mode::Agent => agent::Agent::bind(config),
        &Mode::Master => master::Master::bind(config),
        _ => unreachable!()
    }
}

fn server_pubkey(config: &Config) -> String {
    let server_pubkey = {
        let mut pubkey_path = config.key_path.clone();
        pubkey_path.push("ecpubkey.pem");
        if let Ok(mut file) = File::open(&pubkey_path) {
            let mut key = String::new();
            let _ = file.read_to_string(&mut key);
            key
        } else {
            panic!("You need to supply a server's public key, cannot continue");
        }
    };
    server_pubkey
}

fn connect(host: &str, port: u16, server_publickey: &str) -> ZmqResult<Socket> {
    // info!("Starting zmq sender with version({:?})", zmq::version());
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ)?;
    let client_keypair = zmq::CurveKeyPair::new()?;

    requester.set_curve_serverkey(server_publickey)?;
    requester.set_curve_publickey(&client_keypair.public_key)?;
    requester.set_curve_secretkey(&client_keypair.secret_key)?;
    // info!("Connecting to tcp://{}:{}", host, port);
    assert!(
        requester
            .connect(&format!("tcp://{}:{}", host, port))
            .is_ok()
    );
    // info!("Client mechanism: {:?}", requester.get_mechanism());

    Ok(requester)
}

fn setup_curve(s: &mut Socket, config: &Config) -> ZmqResult<()> {
    // will raise EINVAL if not linked against libsodium
    // The ubuntu package is linked so this shouldn't fail
    s.set_curve_server(true)?;
    let mut pubkey_path = config.key_path.clone();
    pubkey_path.push("ecpubkey.pem");
    let mut key_path = config.key_path.clone();
    key_path.push("ecpubkey.key");
    if let Ok(mut file) = File::open(&key_path) {
        let mut key = String::new();
        let _ = file.read_to_string(&mut key);
        s.set_curve_secretkey(&key)?;
    } else {
        // info!("Creating new curve keypair");
        let keypair = zmq::CurveKeyPair::new()?;
        s.set_curve_secretkey(&keypair.secret_key)?;

        let mut f = File::create(pubkey_path).unwrap();
        f.write(keypair.public_key.as_bytes()).unwrap();
        let mut f = File::create(key_path).unwrap();
        f.write(keypair.secret_key.as_bytes()).unwrap();
    }

    // info!("Server mechanism: {:?}", s.get_mechanism());
    // info!("Curve server: {:?}", s.is_curve_server());

    Ok(())
}

/*
Server that manages disks
*/
fn zmq_listen(
    config: &Arc<RwLock<Config>>,
    callback: Box<Fn(Operation, &Socket) -> ZmqResult<()>>,
) -> ZmqResult<()> {
    trace!("Starting zmq listener with version({:?})", zmq::version());
    let context = zmq::Context::new();
    let mut responder = context.socket(zmq::REP)?;
    {
        let config = config.read().expect("Couldn't setup zmq bind, need read lock on config");
        trace!("Listening on {}", config.zmq_address());
        // Fail to start if this fails
        setup_curve(&mut responder, &config)?;
        assert!(responder.bind(&format!("tcp://{}:{}", config.bind_ip, config.zmq_port)).is_ok());
    }
    trace!("Going into the zmq loop");
    let duration = Duration::from_millis(10);
    loop {
        match responder.recv_bytes(0) {
            Ok(msg) => {
                // info!("Got msg len: {}", msg.len());
                // info!("Parsing msg {:?} as hex", msg);
                let operation = match parse_from_bytes::<Operation>(&msg) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        warn!("Failed to parse_from_bytes {:?}.  Ignoring request", e);
                        continue;
                    }
                };
                // info!("Operation is: {:?}", operation);
                callback(operation, &responder)?
            }
            Err(e) => {
                warn!("Failed to recieve bytes: {:?}", e);
                return Err(e);
            }
        }
        //.expect("Failed to recieve bytes?");


        thread::sleep(duration);
    }
}

pub(crate) fn get_master_pubkey(master: &Master) -> Option<String> {
    if let Ok(mut resp) = reqwest::get(&format!("http://{}:{}/pubkey.pem", master.ip, master.http_port)) {
        let status = resp.status();
        if !status.is_success() {
            return None;
        }

        let mut content = String::new();
        match resp.read_to_string(&mut content) {
            Ok(_size_read) => Some(content),
            Err(e) => {
                info!("error reading server's public key: {:?}", e);
                None
            }
        }
    } else {
        None
    }
}

fn run_plugin(plugin: &PluginEntry, remediation: &Remediation) -> RemediationResult {
    // info!("Hook is: {:?}", hook);
    let mut cmd = Command::new(&plugin.path);
    cmd.arg(format!("plugin={}", plugin.name));
    // cmd.arg(format!("body={}", body));
    for arg in &remediation.args {
        // info!("Adding {} to {:?}", arg, cmd);
        cmd.arg(&arg);
    }
    if let Some(ref alert) = remediation.alert {
        for arg in &alert.args {
            // info!("Adding {} to {:?}", arg, cmd);
            cmd.arg(&arg);
        }
    }
    cmd.arg(format!("remediation={}", encode(&remediation.clone().write_to_bytes().unwrap())));
    // info!("Command is: {:?}", cmd);
    match cmd.output() {
        Ok(output) => {
            match String::from_utf8(output.stdout) {
                Ok(s) => RemediationResult::from_string(&s),
                Err(e) => RemediationResult::new().err(format!("{:?}", e)).with_alert(remediation.alert.clone().unwrap().increment()),
            }
        }
        Err(e) => RemediationResult::new().err(format!("{:?}", e)).with_alert(remediation.alert.clone().unwrap().increment())
    }
}

fn plugins(path: &PathBuf) -> Vec<PluginEntry> {
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
                        Err(e) => warn!("Had a problem loading plugin at {}: {:?}", file.path().display(), e)
                    }
                }
            }
        }
    }
    plugins
}

fn pipelines(config: &Config, plugins: &Vec<PluginEntry>) -> Vec<Pipeline>{
    let pipelines = {
        let pipelines = &config.pipelines;

        match validate(&pipelines, plugins) {
            Ok(()) => {},
            Err(e) => panic!("Couldn't load plugin that matches your pipeline: {:?}", e),
        }

        pipelines.clone()
        
    };
    pipelines
}

fn validate(pipelines: &Vec<Pipeline>, plugins: &Vec<PluginEntry>) -> Result<(), String> {
    for pipeline in pipelines {
        for action in &pipeline.actions {
            debug!("Validating we have a plugin configured for '{}'", action.plugin);
            if !plugins.iter().any(|p| p.name == action.plugin) {
                return Err(format!("{} has no matching plugin", action.plugin));
            }
        }
    }
    Ok(())
}
