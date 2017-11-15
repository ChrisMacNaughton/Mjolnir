extern crate mjolnir_api;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::env;
use std::process;
use std::io::{self, Write};

use serde_json::Error;

use mjolnir_api::{Alert, Message, RepeatedField, RemediationResult, Remediation};
use mjolnir_api::plugin::{Discover, RemediationRequest};

// What does your plugin look like?

fn generate_usage() -> Discover {
    let mut discover = Discover::new();
    discover.set_name("alertmanager".into());
    discover.set_author("Chris MacNaughton <chris@centaurisolutions.nl>".into());
    discover.set_version("0.0.1".into());
    discover.set_webhook(true);
    let mut alerts = Vec::new();
    let mut actions = Vec::new();
    generate_alerts(&mut alerts);
    generate_actions(&mut actions);

    discover.set_actions(Remediation::vec_to_repeated(&actions));
    discover.set_alerts(Alert::vec_to_repeated(&alerts));

    discover
}

// you can plug in actions and alerts below

fn generate_alerts(_alerts: &mut Vec<Alert>) {
    // Your alerts here
}

fn generate_actions(_actions: &mut Vec<Remediation>) {
    // Your actions here
}

// Your plugins should be functions wth this signature

fn alertmanager(args: HashMap<String, String>) -> RemediationResult {
    let body: String = if let Some(body) = args.get("body") {
        if body.len() > 0 {
            body.clone()
        } else {
            return RemediationResult::new().err(format!("Empty Body"))
        }
    } else {
        return RemediationResult::new().err(format!("Missing required argument: Body"))
    };
    let alert: Incoming = match serde_json::from_str(&body) {
        Ok(a) => a,
        Err(e) => return RemediationResult::new().err(format!("Failed to parse json: {:?}", e))
    };

    RemediationResult::new()
        .ok()
        .with_alert(
            Alert {
                alert_type: "alertmanager".into(),
                name: Some(alert.name),
                source: Some(alert.source),
                args: vec![format!("path={}", alert.path)]
            }
        )
}

// You may want custom structs to handle input
#[derive(Serialize, Deserialize)]
struct Incoming {
    source: String,
    path: String,
    name: String,
}

fn main() {
    let plugins = {
        let mut plugins: HashMap<String, _> = HashMap::new();
        // Insert your plugins here!
        plugins.insert("alertmanager".into(), alertmanager);
        plugins
    };

    // Don't touch anything below here!
    let mut arg_list = get_args();

    let plugin = arg_list.remove("plugin").unwrap_or_else(|| {
        println!("Could not find a requested plugin");
        process::exit(1);
    });

    let f = plugins.get(&plugin).unwrap_or_else(|| {
        println!(
            "{} is not a registered plugin, available plugins are: {:?}",
            plugin,
            plugins.keys()
        );
        process::exit(1);
    });

    let res = f(arg_list);
    
    let bytes = res.write_to_bytes().unwrap();

    io::stdout().write(&bytes).unwrap();
}

fn get_args() -> HashMap<String, String> {
    let mut args = env::args();
    if args.len() == 1 {
        // This is the usage directions to Mjolnir
        io::stdout()
            .write(&generate_usage().write_to_bytes().unwrap())
            .unwrap();
        process::exit(0);
    } else {
        let mut arg_list: HashMap<String, String> = HashMap::new();
        let _ = args.next();
        for arg in args {
            let mut parts = arg.split("=");
            let name = parts.next().unwrap().replace("--", "");
            let mut value = parts.next();
            if value.is_none() {
                value = Some("");
            }
            arg_list.insert(name.into(), value.unwrap().into());
        }
        return arg_list;
    }
}
