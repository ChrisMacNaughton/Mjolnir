extern crate mjolnir_api;

use std::collections::HashMap;
use std::env;
use std::process;
use std::io::{self, Write};

use mjolnir_api::{Message, RepeatedField, RemediationResultType};
use mjolnir_api::plugin::{
    Alert, Discover, RemediationRequest, RemediationResult,
};

// What does your plugin look like?

fn generate_usage() -> Discover{
    let mut discover = Discover::new();
    discover.set_name("alertmanager".into());
    discover.set_author("Chris MacNaughton <chris@centaurisolutions.nl>".into());
    discover.set_version("0.0.1".into());
    discover.set_webhook(true);

    generate_alerts(discover.mut_alerts());
    generate_actions(discover.mut_actions());

    discover
}

// you can plug in actions and alerts below

fn generate_alerts(alerts: &mut RepeatedField<Alert>) {
    // Your alerts here
    let mut alert = Alert::new();
    alert.set_title("alertmanager".into());
    alerts.push(alert);
}

fn generate_actions(actions: &mut RepeatedField<RemediationRequest>) {
    // Your actions here
    let mut action = RemediationRequest::new();
    action.set_plugin("alertmanager".into());
    let mut args = RepeatedField::new();
    args.push("body".into());
    action.set_args(args);
    actions.push(action);
}

// Your plugins should be functions wth this signature

fn alertmanager(args: HashMap<String, String>) -> RemediationResult {
    let mut result = RemediationResult::new();
    result.set_result(RemediationResultType::OK);
    // Your plugin action here
    println!("Args for alertmanager are: {:?}", args);
    result
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
        println!("{} is not a registered plugin, available plugins are: {:?}", plugin, plugins.keys());
        process::exit(1);
    });

    f(arg_list);
}

fn get_args() -> HashMap<String, String> {
    let mut args = env::args();
    if args.len() == 1 {
        // println!("Pumping out the protobuf!");

        io::stdout().write(&generate_usage().write_to_bytes().unwrap()).unwrap();
        process::exit(0);
    } else {
        println!("Hello, world!");
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

