extern crate serde;
extern crate serde_json;
extern crate app_dirs;

#[macro_use]
extern crate errln;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

mod eriksync;
mod rsync_command;
mod utils;

use std::io::prelude::*;

use clap::{Arg, App, SubCommand};

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: "eriksync",
    author: "me",
};

fn create_config_file(config_file: &std::path::PathBuf) -> Option<std::path::PathBuf> {

    if config_file.exists() {
        println!("{:?} already exists", config_file);
        return Some(config_file.clone());
    }

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config_file.clone())
        .expect(
            format!("Failed to create {:?}", config_file.as_path()).as_str(),
        );

    match file.write_all(b"{\"nodes\": [], \"targets\": [] }\n") {
        Ok(_) => {
            println!("Created configuration file: {:?}", config_file.as_path());
            Some(config_file.clone())
        }
        Err(e) => {
            errln!("{}", e);
            None
        }
    }
}

fn extract_options(cmd: &clap::ArgMatches) -> (String, Vec<String>) {
    let values = cmd.values_of("").expect("No options.");
    let mut iter = values.into_iter();
    let node = iter.next().expect("No node name.");
    let targets: Vec<String> = iter.map(|t| t.to_string()).collect();
    if targets.is_empty() {
        println!("{}", cmd.usage());
        return (String::new(), Vec::new());
    }

    (node.to_string(), targets)
}

pub fn build_cli() -> App<'static, 'static> {
    App::new("eriksync")
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .takes_value(true)
                .help("The configuration file used by Eriksync"),
        )
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate shell completions")
                .subcommand(SubCommand::with_name("bash").about(
                    "Generate Bash completions",
                ))
                .subcommand(SubCommand::with_name("fish").about(
                    "Generate Fish completions",
                ))
                .subcommand(SubCommand::with_name("zsh").about(
                    "Generate Zsh completions",
                ))
                .subcommand(SubCommand::with_name("powershell").about(
                    "Generate PowerShell completions",
                )),
        )
        .subcommand(SubCommand::with_name("init").about(
            "Initial Eriksync configuration file",
        ))
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
            SubCommand::with_name("dry-push")
                .about("Show commands of push without transfering data")
                .help("node_name [all|target1] [target2]......")
                .arg(Arg::with_name("").multiple(true).takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("dry-pull")
                .about("Show commands of pull without transfering data")
                .help("node_name [all|target1] [target2]......")
                .arg(Arg::with_name("").multiple(true).takes_value(true)),
        )
}

fn main() {

    let mut cli = build_cli();
    let matches = cli.clone().get_matches();

    let config_file = match matches.value_of("config") {
        Some(config_file) => std::path::PathBuf::from(config_file.to_string()),
        None => {
            match app_dirs::get_app_dir(
                app_dirs::AppDataType::UserConfig,
                &APP_INFO,
                "config.json",
            ) {
                Ok(config_file) => config_file,
                Err(_) => {
                    errln!("Failed to get default config file path.");
                    return;
                }
            }
        }
    };

    match matches.subcommand() {
        (_, None) => {
            cli.print_help().unwrap();
        }
        ("completions", Some(cmd)) => {
            let bin_name = "eriksync";
            let shell = match cmd.subcommand() {
                ("bash", Some(_)) => clap::Shell::Bash,
                ("fish", Some(_)) => clap::Shell::Fish,
                ("zsh", Some(_)) => clap::Shell::Zsh,
                ("powershell", Some(_)) => clap::Shell::PowerShell,
                _ => {
                    errln!("{}", cmd.usage());
                    return;
                }
            };
            cli.gen_completions_to(bin_name, shell, &mut std::io::stdout());
        }
        ("init", Some(_)) => {
            create_config_file(&config_file);
        }
        _ => {
            let config = eriksync::Config::load_file(config_file.as_path());
            if config.is_err() {
                errln!(
                    "Failed to load configuration file: {:?}",
                    config_file.as_path()
                );
                return;
            }

            let mut config = config.unwrap();

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
                ("add-node", Some(cmd)) => {
                    let name = cmd.value_of("name").expect("Node name").to_string();
                    let desc = cmd.value_of("description")
                        .expect("Node desctiption")
                        .to_string();
                    config.add_node(eriksync::Node::new(name).description(desc));
                    match config.save_file(config_file.as_path()) {
                        Ok(_) => println!("{}", config.to_json_string()),
                        Err(e) => panic!(e), 
                    }
                }
                ("remove-node", Some(cmd)) => {
                    let name = cmd.value_of("name").expect("Node name").to_string();
                    config.remove_node(name);
                    match config.save_file(config_file.as_path()) {
                        Ok(_) => println!("{}", config.to_json_string()),
                        Err(e) => panic!(e), 
                    }
                }
                ("add-target", Some(cmd)) => {
                    let name = cmd.value_of("name").expect("Target name").to_string();
                    let path = cmd.value_of("path").expect("Target path").to_string();
                    config.add_target(eriksync::Target::new(name, path));
                    match config.save_file(config_file.as_path()) {
                        Ok(_) => println!("{}", config.to_json_string()),
                        Err(e) => panic!(e), 
                    }
                }
                ("remove-target", Some(cmd)) => {
                    let name = cmd.value_of("name").expect("Target name").to_string();
                    config.remove_target(name);
                    match config.save_file(config_file.as_path()) {
                        Ok(_) => println!("{}", config.to_json_string()),
                        Err(e) => panic!(e), 
                    }
                }
                ("push", Some(cmd)) => {
                    let (node, targets) = extract_options(cmd);
                    rsync_command::run_commands(&rsync_command::generate_commands(
                        &config,
                        &node,
                        &targets,
                        rsync_command::Direction::Push,
                    ));
                }
                ("pull", Some(cmd)) => {
                    let (node, targets) = extract_options(cmd);
                    rsync_command::run_commands(&rsync_command::generate_commands(
                        &config,
                        &node,
                        &targets,
                        rsync_command::Direction::Pull,
                    ));
                }
                ("dry-push", Some(cmd)) => {
                    let (node, targets) = extract_options(cmd);
                    rsync_command::show_commands(&rsync_command::generate_commands(
                        &config,
                        &node,
                        &targets,
                        rsync_command::Direction::Push,
                    ));
                }
                ("dry-pull", Some(cmd)) => {
                    let (node, targets) = extract_options(cmd);
                    rsync_command::show_commands(&rsync_command::generate_commands(
                        &config,
                        &node,
                        &targets,
                        rsync_command::Direction::Pull,
                    ));
                }
                _ => {
                    cli.print_help().unwrap();
                }
            }
        }
    }
}
