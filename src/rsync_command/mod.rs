use std;
use eriksync;
use utils;

pub struct RsyncCommand {
    command: String,
    arguements: Vec<String>,
}

pub enum Direction {
    Push,
    Pull,
}

pub fn show_commands(commands: &Vec<RsyncCommand>) {
    for cmd in commands {
        println!("{} {:?}", cmd.command, cmd.arguements);
    }
}

pub fn run_commands(commands: &Vec<RsyncCommand>) {
    show_commands(commands);
    for cmd in commands {
        let mut child = std::process::Command::new(cmd.command.clone())
            .args(cmd.arguements.as_slice())
            .spawn()
            .expect("Failed to execute child.");

        let exit_code = child.wait().expect("failed to wait on child");
        if !exit_code.success() {
            break;
        }
    }
}

pub fn generate_commands(
    config: &eriksync::Config,
    node_name: &String,
    target_list: &Vec<String>,
    direction: Direction,
) -> Vec<RsyncCommand> {

    if !config.contains_node(node_name) || target_list.is_empty() {
        return Vec::new();
    }

    let targets = if target_list[0].to_lowercase() == "all" {
        config.target_names()
    } else {
        target_list.to_vec()
    };

    let command_getter = match direction {
        Direction::Push => push_command,
        Direction::Pull => pull_command,
    };

    let node = config.get_node(node_name).expect("No such node");
    targets
        .iter()
        .filter(|t| config.contains_target(t))
        .map(|t| command_getter(node, config.get_target(t).unwrap()))
        .collect()
}

pub fn push_command(node: &eriksync::Node, target: &eriksync::Target) -> RsyncCommand {
    let (local_dir, remote_dir) = get_target_pair(node, target);
    get_command(local_dir, remote_dir)
}

pub fn pull_command(node: &eriksync::Node, target: &eriksync::Target) -> RsyncCommand {
    let (local_dir, remote_dir) = get_target_pair(node, target);
    get_command(remote_dir, local_dir)
}

fn get_command(src: String, dest: String) -> RsyncCommand {
    RsyncCommand {
        command: "rsync".to_string(),
        arguements: vec![
            String::from("-avzHSP"),
            String::from("--delete"),
            String::from("-e"),
            String::from("ssh"),
            src,
            dest,
        ],
    }
}

fn get_target_pair(node: &eriksync::Node, target: &eriksync::Target) -> (String, String) {
    let remote_dir = format!("{}:{}", node.name, target.path);
    let local_dir = utils::expand_user(&std::path::Path::new(&target.path));

    (
        String::from(local_dir.to_str().unwrap_or_default()),
        remote_dir,
    )
}
