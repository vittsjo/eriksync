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

use clap::{Arg, App, SubCommand};

use eriksync::config;

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: crate_name!(),
    author: "me",
};

fn create_config_file(
    config_file: &std::path::PathBuf,
    format: config::ConfigFormat,
) -> Option<std::path::PathBuf> {

    if config_file.exists() {
        println!("{:?} already exists", config_file);
        return Some(config_file.clone());
    }

    let config = config::Config::new();
    match config.save_file(config_file, format) {
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
    let arg_format = Arg::with_name("format")
        .long("format")
        .takes_value(true)
        .help("format of configuration file");

    let arg_replace = Arg::with_name("replace").long("replace").short("r").help(
        "replace files/folders if they already exist",
    );


    App::new(crate_name!())
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
        .arg(arg_format.clone())
        .subcommand(SubCommand::with_name("version").about(
            "Show version of Eriksync",
        ))
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
        .subcommand(
            SubCommand::with_name("init")
                .about("Initial Eriksync configuration file")
                .arg(arg_replace.clone())
                .arg(arg_format.clone()),
        )
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
        .subcommand(
            SubCommand::with_name("show-config")
                .about("Print configuration")
                .arg(arg_format.clone()),
        )
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

pub fn extract_format(cmd: &clap::ArgMatches) -> config::ConfigFormat {
    cmd.value_of("format").map_or(
        config::ConfigFormat::yaml,
        |format| {
            config::ConfigFormat::from_str(format).unwrap_or(config::ConfigFormat::yaml)
        },
    )
}

pub fn save_config(config: &config::Config, format: config::ConfigFormat, path: &std::path::Path) {
    match config.save_file(path, format) {
        Ok(_) => println!("{}", config.to_json_string()),
        Err(e) => panic!(e), 
    }
}

fn default_config_file_path(format: config::ConfigFormat) -> std::path::PathBuf {
    app_dirs::get_app_dir(
        app_dirs::AppDataType::UserConfig,
        &APP_INFO,
        format!("{}.{}", crate_name!(), format.to_string()).as_str(),
    ).map_err(|e| errln!("{}", e.to_string()))
        .unwrap()
}

fn main() {

    let mut cli = build_cli();
    let matches = cli.clone().get_matches();

    match matches.subcommand() {
        (_, None) => {
            cli.print_help().unwrap();
        }
        ("version", Some(_)) => {
            println!("{} {}", crate_name!(), crate_version!());
        }
        ("completions", Some(cmd)) => {
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
            cli.gen_completions_to(crate_name!(), shell, &mut std::io::stdout());
        }
        ("init", Some(cmd)) => {
            let config_file = default_config_file_path(config::ConfigFormat::yaml);
            create_config_file(&config_file, extract_format(cmd));
        }
        _ => {
            let format = extract_format(&matches);
            let config_file = match matches.value_of("config") {
                Some(config_file) => std::path::PathBuf::from(config_file.to_string()),
                None => {
                    if default_config_file_path(config::ConfigFormat::yaml).exists() {
                        default_config_file_path(config::ConfigFormat::yaml)
                    } else if default_config_file_path(config::ConfigFormat::toml).exists() {
                        default_config_file_path(config::ConfigFormat::toml)
                    } else if default_config_file_path(config::ConfigFormat::json).exists() {
                        default_config_file_path(config::ConfigFormat::json)
                    } else {
                        default_config_file_path(config::ConfigFormat::yaml)
                    }
                }
            };

            let mut config = match eriksync::Config::load_file(config_file.as_path()) {
                Ok(config) => config,
                Err(e) => {
                    errln!(
                        "Failed to load configuration file: {:?}, error: {:?}",
                        config_file.as_path(),
                        e
                    );
                    return;
                } 
            };

            match matches.subcommand() {
                ("config-location", Some(_)) => {
                    println!("{:?}", config_file);
                }
                ("show-config", Some(cmd)) => {
                    let format = extract_format(&cmd);
                    println!(
                        "{}",
                        match format {
                            config::ConfigFormat::json => config.to_json_string(),
                            config::ConfigFormat::yaml => config.to_yaml_string(),
                            config::ConfigFormat::toml => config.to_toml_string(),
                        }
                    );
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
                    save_config(&config, format, config_file.as_path());
                }
                ("remove-node", Some(cmd)) => {
                    let name = cmd.value_of("name").expect("Node name").to_string();
                    config.remove_node(name);
                    save_config(&config, format, config_file.as_path());
                }
                ("add-target", Some(cmd)) => {
                    let name = cmd.value_of("name").expect("Target name").to_string();
                    let path = cmd.value_of("path").expect("Target path").to_string();
                    config.add_target(eriksync::Target::new(name, path));
                    save_config(&config, format, config_file.as_path());
                }
                ("remove-target", Some(cmd)) => {
                    let name = cmd.value_of("name").expect("Target name").to_string();
                    config.remove_target(name);
                    save_config(&config, format, config_file.as_path());
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
