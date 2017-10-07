extern crate serde;
extern crate serde_json;

use std;
use std::io;
use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub name: String,
    pub description: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Target {
    pub name: String,
    pub path: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Config {
    pub file_path: String,
    pub nodes: HashMap<String, Node>,
    pub targets: HashMap<String, Target>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct JSONConfig {
    nodes: Vec<Node>,
    targets: Vec<Target>,
}

#[allow(dead_code)]
impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name: name,
            description: String::new(),
        }
    }

    pub fn description(mut self, description: String) -> Node {
        self.description = description;
        self
    }
}

#[allow(dead_code)]
impl Target {
    pub fn new(name: String, path: String) -> Target {
        Target {
            name: name,
            path: path,
        }
    }
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Self {
        Config {
            file_path: String::new(),
            nodes: HashMap::new(),
            targets: HashMap::new(),
        }
    }

    pub fn load_file(path: &std::path::Path) -> io::Result<Self> {
        match File::open(path) {
            Err(why) => Err(why),
            Ok(file) => {
                let raw: JSONConfig = serde_json::from_reader(file).unwrap();
                let mut config = Config::new();
                config.file_path = path.to_str().unwrap().to_string();
                config.add_nodes(raw.nodes);
                config.add_targets(raw.targets);

                Ok(config)
            }
        }
    }

    pub fn save_file(&self, path: &std::path::Path) -> io::Result<()> {
        match File::create(path) {
            Ok(mut file) => file.write_all(self.to_json_string().as_bytes()),
            Err(e) => Err(e),
        }
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(&JSONConfig {
            nodes: self.nodes.iter().map(|(_, n)| n.clone()).collect(),
            targets: self.targets.iter().map(|(_, t)| t.clone()).collect(),
        }).unwrap_or_default()
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.name.to_owned(), node);
    }

    pub fn add_nodes(&mut self, nodes: Vec<Node>) {
        for n in nodes {
            self.add_node(n);
        }
    }

    pub fn add_target(&mut self, target: Target) {
        self.targets.insert(target.name.to_owned(), target);
    }

    pub fn add_targets(&mut self, targets: Vec<Target>) {
        for t in targets {
            self.add_target(t);
        }
    }

    pub fn remove_node(&mut self, node_name: String) {
        self.nodes.remove(&node_name);
    }

    pub fn remove_target(&mut self, target_name: String) {
        self.targets.remove(&target_name);
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.nodes.iter().map(|(_, v)| v.clone()).collect()
    }

    pub fn node_names(&self) -> Vec<String> {
        self.nodes.iter().map(|(_, v)| v.name.clone()).collect()
    }

    pub fn targets(&self) -> Vec<Target> {
        self.targets.iter().map(|(_, v)| v.clone()).collect()
    }

    pub fn target_names(&self) -> Vec<String> {
        self.targets.iter().map(|(_, v)| v.name.clone()).collect()
    }

    pub fn get_node(&self, node_name: &String) -> Option<&Node> {
        self.nodes.get(node_name)
    }

    pub fn get_target(&self, target_name: &String) -> Option<&Target> {
        self.targets.get(target_name)
    }

    pub fn contains_node(&self, node_name: &String) -> bool {
        self.nodes.get(node_name).is_some()
    }

    pub fn contains_target(&self, target_name: &String) -> bool {
        self.targets.get(target_name).is_some()
    }
}


mod test {
    extern crate serde;
    extern crate serde_json;

    #[cfg(test)]
    fn config_test() {

        let mut config = eriksync::Config::new();

        println!("{}", serde_json::to_string(&config).unwrap());

        config.add_node(eriksync::Node::new("xapek".to_string()).description(
            "PLSM Lab".to_string(),
        ));
        config.add_node(eriksync::Node::new("xapek".to_string()).description(
            "PLSM Lab".to_string(),
        ));
        println!("{}", serde_json::to_string(&config).unwrap());

        config.add_node(eriksync::Node::new("vapmi".to_string()).description(
            "MacBook Pro".to_string(),
        ));
        println!("{}", serde_json::to_string(&config).unwrap());

        config.add_target(eriksync::Target::new(
            "dotfiles".to_string(),
            "~/dotfiles/".to_string(),
        ));
        println!("{}", serde_json::to_string(&config).unwrap());


        config.add_target(eriksync::Target::new(
            "dotfiles".to_string(),
            "~/dotfiles/".to_string(),
        ));
        println!("{}", serde_json::to_string(&config).unwrap());
    }

}
