extern crate serde;
extern crate serde_json;
extern crate app_dirs;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

mod eriksync;
mod rsync_command;
mod utils;
mod cli;

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

    let matches = cli::build_cli().get_matches();
    let bin_name = "eriksync";

    match matches.subcommand() {
        ("completions", Some(cmd)) => {
            let shell = match cmd.subcommand() {
                ("bash", Some(_)) => clap::Shell::Bash,
                ("fish", Some(_)) => clap::Shell::Fish,
                ("zsh", Some(_)) => clap::Shell::Zsh,
                ("powershell", Some(_)) => clap::Shell::PowerShell,
                _ => {
                    println!("{}", cmd.usage());
                    return;
                }
            };
            cli::build_cli().gen_completions_to(bin_name, shell, &mut std::io::stdout());
        }
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
