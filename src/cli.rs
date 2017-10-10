
use clap::{Arg, App, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("eriksync")
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

}
