use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone)]
pub struct IndexStorage {
    pub index: HashMap<String, Vec<String>>,
}

impl IndexStorage {
    pub fn new() -> IndexStorage {
        IndexStorage {
            index: HashMap::new(),
        }
    }

    pub fn store_processed_text_in_index(&mut self, text: &str, path: &Path) {
        let path_str = path.to_str().unwrap().to_owned();

        // Tokenize the processed text
        let tokens = text.split_whitespace().collect::<Vec<&str>>();

        // Insert each token into the index
        for token in tokens {
            self.index
                .entry(token.to_string())
                .or_insert_with(Vec::new)
                .push(path_str.clone());
        }
    }

    pub fn save_index_to_json_file(&self, output_path: &Path) -> std::io::Result<()> {
        let json_data = serde_json::to_string_pretty(&self.index)?;
        let mut file = File::create(output_path)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }
}
