#[macro_use]
extern crate mjolnir_api;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::io::{self, Write};

use mjolnir_api::{Alert, Discover, RemediationResult, Remediation};

// What does your plugin look like?

fn generate_usage() -> Discover {
    Discover::new("clean_disk")
        .with_author("Chris MacNaughton <chris@centaurisolutions.nl>")
        .with_version("0.0.1")
        .with_alerts(generate_alerts())
        .with_remediations(generate_actions())
        .webhook()
}

// you can plug in actions and alerts below

fn generate_alerts() -> Vec<Alert> {
    // Your alerts here
    vec![]
}

fn generate_actions() -> Vec<Remediation> {
    // Your actions here
    vec![Remediation {
        plugin: "clean_disk".into(),
        target: None,
        args: vec!["path".into()],
        alert: None,
    }]
}

fn list_plugins() -> HashMap<String, fn(HashMap<String, String>) -> RemediationResult> {
    plugin_list!("clean_disk" => clean)
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
                Err(e) => return result.err(format!("couldn't read directory {}: {:?}", s, e)),
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
