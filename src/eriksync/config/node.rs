extern crate serde;
extern crate serde_json;

use std::cmp::Ordering;

#[allow(dead_code)]
#[derive(Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    #[serde(skip)]
    pub name: String,

    #[serde(default)]
    pub description: String,
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

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.name == other.name
    }
}
