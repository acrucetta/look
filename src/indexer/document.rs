use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub struct Document {
    pub path: String,
}

impl Document {
    pub fn new(path: String) -> Document {
        Document { path }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct Term(pub String);
