use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub struct Document {
    pub path: String,
    pub contents: String,
}

impl Document {
    pub fn new(path: String, contents: String) -> Document {
        Document { path, contents }
    }
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
