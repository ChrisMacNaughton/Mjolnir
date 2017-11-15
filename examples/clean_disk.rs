extern crate mjolnir_api;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::io::{self, Write};

use mjolnir_api::{Alert, Message, RemediationResult, Remediation};
use mjolnir_api::plugin::{Discover};

// What does your plugin look like?

fn generate_usage() -> Discover {
    let mut discover = Discover::new();
    discover.set_name("clean_disk".into());
    discover.set_author("Chris MacNaughton <chris@centaurisolutions.nl>".into());
    discover.set_version("0.0.1".into());
    discover.set_webhook(false);
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

fn generate_actions(actions: &mut Vec<Remediation>) {
    // Your actions here
    actions.push(Remediation {
        plugin: "clean_disk".into(),
        target: None,
        args: vec!["path".into()],
        alert: None,
    });
    // actions.push(action.into());
}

// Your plugins should be functions wth this signature

fn clean(args: HashMap<String, String>) -> RemediationResult {
    let result = RemediationResult::new();
    match args.get("path") {
        Some(s) => {
            match fs::read_dir(s) {
                Ok(dir) => {
                    let mut has_failed = false;
                    let mut failed_entry = String::new();
                    for entry in dir {
                        if let Ok(entry) = entry {
                            let path = entry.path();

                            if path.is_dir() {
                                if let Err(e) = fs::remove_dir_all(&path) {
                                    if has_failed == false {
                                        has_failed = true;
                                        failed_entry = format!("Failed to remove {}: {:?}", path.display(), e);
                                    }
                                }
                            } else {
                                if let Err(e) = fs::remove_file(&path) {
                                    if has_failed == false {
                                        has_failed = true;
                                        failed_entry = format!("Failed to remove {}: {:?}", path.display(), e);
                                    }
                                }
                            }
                        }
                    }
                    if has_failed {
                        return result.err(failed_entry);
                    }
                },
                Err(e) => return result.err(format!("couldn't reead directory {}: {:?}", s, e)),
            }
        },
        None => {
            return result.err("Missing required argument: Path");
        }
    }
    // result.set_result(RemediationResultType::OK);
    // Your plugin action here
    // println!("Running the clean plugin with args: {:?}", args);
    result
}

fn main() {
    let plugins = {
        let mut plugins: HashMap<String, _> = HashMap::new();
        // Insert your plugins here!
        plugins.insert("clean_disk".into(), clean);
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
