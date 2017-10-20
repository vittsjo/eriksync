pub mod node;
pub mod target;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

use std;
use std::io::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;

pub use self::node::Node;
pub use self::target::Target;

#[derive(Debug)]
pub enum ConfigFormat {
    yaml,
    json,
    toml,
}

impl std::fmt::Display for ConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ConfigFormat {
    pub fn from_str(s: &str) -> Option<ConfigFormat> {
        match s.to_lowercase().as_str() {
            "json" => Some(ConfigFormat::json),
            "toml" => Some(ConfigFormat::toml),
            "yaml" | "yml" => Some(ConfigFormat::yaml),
            _ => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub nodes: HashMap<String, Node>,

    #[serde(default)]
    pub targets: HashMap<String, Target>,
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Self {
        Config {
            nodes: HashMap::new(),
            targets: HashMap::new(),
        }
    }

    pub fn load_file(path: &std::path::Path) -> std::result::Result<Self, String> {

        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let ext = match path.extension() {
            Some(ext) => ext.to_str().unwrap_or(&""),
            None => {
                return Err(format!("{:?} has no extension name", path));
            } 
        };

        let config: Config = match ConfigFormat::from_str(ext) {
            Some(ConfigFormat::yaml) => serde_yaml::from_reader(file).map_err(|e| e.to_string()),
            Some(ConfigFormat::json) => serde_json::from_reader(file).map_err(|e| e.to_string()),
            Some(ConfigFormat::toml) => serde_json::from_reader(file).map_err(|e| e.to_string()),
            None => {
                return Err(format!("{:?} is not valid Unicode string", ext));
            }
        }.unwrap();

        let mut ret = Config::new();

        for (name, mut node) in config.nodes {
            node.name = name;
            ret.add_node(node);
        }

        for (name, mut target) in config.targets {
            target.name = name;
            ret.add_target(target);
        }

        Ok(ret)
    }

    pub fn save_file(&self, path: &std::path::Path, format: ConfigFormat) -> std::io::Result<()> {
        match std::fs::File::create(path) {
            Err(e) => Err(e),
            Ok(mut file) => {
                file.write_all(
                    match format {
                        ConfigFormat::json => self.to_json_string(),
                        ConfigFormat::yaml => self.to_yaml_string(),
                        ConfigFormat::toml => self.to_toml_string(),
                    }.as_bytes(),
                )
            }
        }
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn to_toml_string(&self) -> String {
        toml::to_string(self).unwrap_or_default()
    }

    pub fn to_yaml_string(&self) -> String {
        serde_yaml::to_string(self).unwrap_or_default()
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

    pub fn remove_nodes(&mut self, node_names: Vec<String>) {
        for n in node_names {
            self.remove_node(n);
        }
    }

    pub fn remove_target(&mut self, target_name: String) {
        self.targets.remove(&target_name);
    }

    pub fn remove_targets(&mut self, target_names: Vec<String>) {
        for t in target_names {
            self.remove_target(t);
        }
    }

    pub fn nodes(&self) -> Vec<Node> {
        let mut nodes: Vec<Node> = self.nodes.iter().map(|(_, v)| v.clone()).collect();
        nodes.sort();
        nodes
    }

    pub fn node_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.nodes.iter().map(|(_, v)| v.name.clone()).collect();
        names.sort();
        names
    }

    pub fn targets(&self) -> Vec<Target> {
        let mut targets: Vec<Target> = self.targets.iter().map(|(_, v)| v.clone()).collect();
        targets.sort();
        targets
    }

    pub fn target_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.targets.iter().map(|(_, v)| v.name.clone()).collect();
        names.sort();
        names
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
