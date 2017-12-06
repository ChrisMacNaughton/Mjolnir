#[macro_use]
extern crate mjolnir_api;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::env;
use std::process;
use std::io::{self, Write};

use mjolnir_api::{Alert, Discover, RemediationResult, Remediation};

#[cfg(test)]
mod tests {
    use super::*;

    fn it_parses_alertmanager_json() {
        let json = r#"{
    "groupLabels": {"alertname": "full-disk"},
    "groupKey": "test",
    "commonLabels": {"path": "/tmp/test", "alertname": "full-disk", "host": "10.0.1.10"},
    "commonAnnotations": {},
    "externalURL": "http://alertmanager.local",
    "receiver": "malerts",
    "version": "4",
    "status": "firing",
    "alerts": [{
        "labels": {"path": "/tmp/test", "alertname": "full-disk", "host": "10.0.1.10"},
        "status": "firing",
        "annotations": {},
        "generatorURL": "http://prometheus.local/graph?...",
        "startsAt": "2017-01-01T00:00:00.000Z",
        "endsAt": "0001-01-01T00:00:00Z"
    }]
}"#;
        let alert: Incoming = serde_json::from_str(&json).unwrap();
    }
}

// What does your plugin look like?

fn generate_usage() -> Discover {
    Discover::new("alertmanager")
        .with_author("Chris MacNaughton <chris@centaurisolutions.nl>")
        .with_version("0.0.1")
        .with_alerts(generate_alerts())
        .with_remediations(generate_actions())
        .webhook()
}

// you can plug in actions and alerts below

fn generate_alerts() -> Vec<Alert> {
    // Your alerts here
    vec![Alert::new("alertmanager")]
}

fn generate_actions() -> Vec<Remediation> {
    // Your actions here
    vec![]
}

fn list_plugins() -> HashMap<String, fn(HashMap<String, String>) -> RemediationResult> {
    // This is an exmaple of what the below macro expands into
    //
    // let mut plugins: HashMap<String, _> = HashMap::new();
    // plugins.insert(
    //    "alertmanager".into(), alertmanager as fn(HashMap<String, String>) -> RemediationResult
    // );
    // plugins

    // Insert your plugins here!
    plugin_list!("alertmanager" => alertmanager)
}

// Your plugins should be functions wth this signature

fn alertmanager(args: HashMap<String, String>) -> RemediationResult {
    let body: String = if let Some(body) = args.get("body") {
        if body.len() > 0 {
            body.clone()
        } else {
            return RemediationResult::new().err(format!("Empty Body"));
        }
    } else {
        return RemediationResult::new().err(format!("Missing required argument: Body"));
    };
    let incoming: Incoming = match serde_json::from_str(&body) {
        Ok(a) => a,
        Err(e) => return RemediationResult::new().err(format!("Failed to parse json: {:?}", e)),
    };
    let alerts = incoming
        .alerts
        .iter()
        .map(|a| {
            let mut alert = Alert::new("alertmanager");
            alert = alert.with_arg(format!("raw={:?}", incoming));
            if let Some(name) = a.labels.get("alertname") {
                alert = alert.with_name(name.clone());
            }
            if let Some(host) = a.labels.get("host") {
                alert = alert.with_source(host.clone());
            }
            for (key, value) in &a.labels {
                alert = alert.with_arg(format!("{}={}", key, value));
            }
            alert
        })
        .collect();
    RemediationResult::new()
        .ok()
        // .with_alert(
        //     Alert::new("alertmanager")
        //         .with_name(alert.name)
        //         .with_source(alert.source)
        //         .with_args(vec![format!("path={}", alert.path)])
        // )
        .with_alerts(alerts)
}

// You may want custom structs to handle input
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
struct Incoming {
    version: String,
    group_key: String,
    status: Status,
    receiver: String,
    group_labels: HashMap<String, String>,
    common_labels: HashMap<String, String>,
    common_annotations: HashMap<String, String>,
    externalURL: String,
    alerts: Vec<PAlert>,
    // source: String,
    // path: String,
    // name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
enum Status {
    resolved,
    firing,
}

#[derive(Debug, Serialize, Deserialize)]
struct PAlert {
    labels: HashMap<String, String>,
    annotations: HashMap<String, String>,
    // startsAt: String,
    // endsAt: String,
}

// Don't touch anything below here!

fn main() {
    let plugins = list_plugins();

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

    io::stdout().write(&res.write_to_bytes().unwrap()).unwrap();
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
