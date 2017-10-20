extern crate serde;
extern crate serde_json;

use std::cmp::Ordering;

#[allow(dead_code)]
#[derive(Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Target {
    #[serde(skip)]
    pub name: String,

    #[serde(default)]
    pub path: String,
}

#[allow(dead_code)]
impl Target {
    pub fn new(name: String, path: String) -> Self {
        Target {
            name: name,
            path: path,
        }
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Target) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Target {
    fn cmp(&self, other: &Target) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Target) -> bool {
        self.name == other.name
    }
}
