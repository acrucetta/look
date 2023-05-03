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

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Only display the filename, not the full path
        let path = std::path::Path::new(&self.path);
        let filename = path.file_name().unwrap().to_str().unwrap();
        write!(f, "{}", filename)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct Term(pub String);
