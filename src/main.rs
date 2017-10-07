extern crate serde;
extern crate serde_json;
extern crate app_dirs;

#[macro_use]
extern crate serde_derive;
use clap::{Arg, App, SubCommand};

#[macro_use]
extern crate clap;

mod eriksync;
mod rsync_command;
mod utils;

use std::io::prelude::*;

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: "eriksync",
    author: "me",
};

fn create_config_file() -> std::path::PathBuf {
    let config_file =
        match app_dirs::get_app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "config.json") { 
            Ok(path) => path,
            Err(_) => {
                app_dirs::app_dir(app_dirs::AppDataType::UserConfig, &APP_INFO, "config.json")
                    .expect(&format!(
                        "Failed to create {:?}",
                        app_dirs::get_app_dir(
                            app_dirs::AppDataType::UserConfig,
                            &APP_INFO,
                            "config.json",
                        )
                    ))
            }
        };

    if !config_file.exists() {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(config_file.clone())
            .expect(
                format!("Failed to create {:?}", config_file.as_path()).as_str(),
            );

        match file.write_all(b"{\"nodes\": [], \"targets\": [] }\n") {
            Ok(_) => println!("Create configuration file: {:?}", config_file.as_path()),
            Err(e) => println!("{}", e),
        }
        drop(file);
    }

    config_file
}

fn extract_options(sub_m: &clap::ArgMatches) -> (String, Vec<String>) {
    let values = sub_m.values_of("").expect("No options.");
    let mut iter = values.into_iter();
    let node = iter.next().expect("No node name.");
    let targets: Vec<String> = iter.map(|t| t.to_string()).collect();
    if targets.is_empty() {
        println!("{}", sub_m.usage());
        return (String::new(), Vec::new());
    }

    (node.to_string(), targets)
}

fn main() {
    let config_file = create_config_file();
    let mut config = eriksync::Config::load_file(config_file.as_path()).unwrap();

    let matches = App::new("eriksync")
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("add-node")
                .about("Add node")
                .arg(Arg::with_name("name").long("name").short("n").takes_value(
                    true,
                ))
                .arg(
                    Arg::with_name("description")
                        .long("description")
                        .short("d")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove-node")
                .about("Remove node")
                .arg(Arg::with_name("name").long("name").short("n").takes_value(
                    true,
                )),
        )
        .subcommand(
            SubCommand::with_name("add-target")
                .about("Add target")
                .arg(Arg::with_name("name").long("name").short("n").takes_value(
                    true,
                ))
                .arg(Arg::with_name("path").long("path").short("p").takes_value(
                    true,
                )),
        )
        .subcommand(
            SubCommand::with_name("remove-target")
                .about("Remove target")
                .arg(Arg::with_name("name").long("name").short("n").takes_value(
                    true,
                )),
        )
        .subcommand(SubCommand::with_name("list-nodes").about("Print nodes"))
        .subcommand(SubCommand::with_name("list-targets").about("Print targets"))
        .subcommand(SubCommand::with_name("config-location").about(
            "Print location of configuration file",
        ))
        .subcommand(SubCommand::with_name("show-config").about(
            "Print configuration",
        ))
        .subcommand(
            SubCommand::with_name("push")
                .about("Send data from local host to remote host")
                .help("node_name [all|target1] [target2]......")
                .arg(Arg::with_name("").multiple(true).takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("pull")
                .about("Send data from remote host to local host")
                .help("node_name [all|target1] [target2]......")
                .arg(Arg::with_name("").multiple(true).takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("push-command")
                .about("Show commands of push without transfering data")
                .help("node_name [all|target1] [target2]......")
                .arg(Arg::with_name("").multiple(true).takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("pull-command")
                .about("Show commands of pull without transfering data")
                .help("node_name [all|target1] [target2]......")
                .arg(Arg::with_name("").multiple(true).takes_value(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("config-location", Some(_)) => {
            println!("{}", config.file_path);
        }
        ("show-config", Some(_)) => {
            println!("{}", config.to_json_string());
        }
        ("list-nodes", Some(_)) => {
            for node in config.nodes() {
                println!("{}: {}", node.name, node.description);
            }
        }
        ("list-targets", Some(_)) => {
            for target in config.targets() {
                println!("{}: {}", target.name, target.path);
            }
        }
        ("add-node", Some(sub_m)) => {
            let name = sub_m.value_of("name").expect("Node name").to_string();
            let desc = sub_m
                .value_of("description")
                .expect("Node desctiption")
                .to_string();
            config.add_node(eriksync::Node::new(name).description(desc));
            match config.save_file(config_file.as_path()) {
                Ok(_) => println!("{}", config.to_json_string()),
                Err(e) => panic!(e), 
            }
        }
        ("remove-node", Some(sub_m)) => {
            let name = sub_m.value_of("name").expect("Node name").to_string();
            config.remove_node(name);
            match config.save_file(config_file.as_path()) {
                Ok(_) => println!("{}", config.to_json_string()),
                Err(e) => panic!(e), 
            }
        }
        ("add-target", Some(sub_m)) => {
            let name = sub_m.value_of("name").expect("Target name").to_string();
            let path = sub_m.value_of("path").expect("Target path").to_string();
            config.add_target(eriksync::Target::new(name, path));
            match config.save_file(config_file.as_path()) {
                Ok(_) => println!("{}", config.to_json_string()),
                Err(e) => panic!(e), 
            }
        }
        ("remove-target", Some(sub_m)) => {
            let name = sub_m.value_of("name").expect("Target name").to_string();
            config.remove_target(name);
            match config.save_file(config_file.as_path()) {
                Ok(_) => println!("{}", config.to_json_string()),
                Err(e) => panic!(e), 
            }
        }
        ("push", Some(sub_m)) => {
            let (node, targets) = extract_options(sub_m);
            rsync_command::run_commands(&rsync_command::generate_commands(
                &config,
                &node,
                &targets,
                rsync_command::Direction::Push,
            ));
        }
        ("pull", Some(sub_m)) => {
            let (node, targets) = extract_options(sub_m);
            rsync_command::run_commands(&rsync_command::generate_commands(
                &config,
                &node,
                &targets,
                rsync_command::Direction::Pull,
            ));
        }
        ("push-command", Some(sub_m)) => {
            let (node, targets) = extract_options(sub_m);
            rsync_command::show_commands(&rsync_command::generate_commands(
                &config,
                &node,
                &targets,
                rsync_command::Direction::Push,
            ));
        }
        ("pull-command", Some(sub_m)) => {
            let (node, targets) = extract_options(sub_m);
            rsync_command::show_commands(&rsync_command::generate_commands(
                &config,
                &node,
                &targets,
                rsync_command::Direction::Pull,
            ));
        }
        _ => println!("{}", matches.usage()),
    }
}
