use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Term(String);

#[derive(Clone, Serialize, Deserialize)]
pub struct Index {
    pub inverted_index: HashMap<Term, HashMap<String, u32>>,
    pub idf: HashMap<Term, f64>,
    num_docs: usize,
}

impl Index {
    pub fn new() -> Index {
        Index {
            inverted_index: HashMap::new(),
            idf: HashMap::new(),
            num_docs: 0,
        }
    }

    pub fn store_processed_text_in_index(&mut self, text: &str, path: &Path) {
        let path_str = path.to_str().unwrap().to_owned();

        // Tokenize the processed text
        let tokens = text.split_whitespace().collect::<Vec<&str>>();

        // Increment the number of documents
        self.num_docs += 1;

        // Insert each token into the index and calculate term frequency
        let mut term_counts = HashMap::new();
        for token in tokens {
            let term = Term(token.to_owned());
            let count = term_counts.entry(term.clone()).or_insert(0);
            *count += 1;
        }

        for (term, count) in term_counts {
            self.inverted_index
                .entry(term)
                .or_insert_with(HashMap::new)
                .insert(path_str.clone(), count);
        }
    }

    pub fn calculate_idf(&mut self) {
        for (term, docs) in &self.inverted_index {
            let idf = (self.num_docs as f64 / docs.len() as f64).ln();
            self.idf.insert(term.clone(), idf);
        }
    }

    pub fn save_index_to_json_file(&self, output_path: &Path) -> std::io::Result<()> {
        let json_data = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(output_path)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::indexer::index_storage::Term;

    #[test]
    fn test_store_processed_text_in_index() {
        use super::Index;
        use std::path::Path;

        let mut index = Index::new();
        let text = "This is a test sentence.";
        let path = Path::new("test.txt");
        index.store_processed_text_in_index(text, path);

        assert_eq!(index.num_docs, 1);
        assert_eq!(index.inverted_index.len(), 5);

        // Create a HashMap of the expected results
        let mut expected: HashMap<Term, HashMap<String, u32>> = HashMap::new();
        for term in vec![
            Term("This".to_owned()),
            Term("is".to_owned()),
            Term("a".to_owned()),
            Term("test".to_owned()),
            Term("sentence.".to_owned()),
        ] {
            let mut docs = HashMap::new();
            docs.insert("test.txt".to_owned(), 1);
            expected.insert(term, docs);
        }

        for (term, docs) in &expected {
            assert_eq!(index.inverted_index.get(term), Some(docs));
        }
    }

    #[test]
    fn test_calculate_idf() {
        use super::Index;
        use std::path::Path;

        let mut index = Index::new();
        let text = "This is a test sentence.";
        let path = Path::new("test.txt");
        index.store_processed_text_in_index(text, path);
        index.calculate_idf();

        assert_eq!(index.idf.len(), 5);

        // Create a HashMap of the expected results
        let mut expected: HashMap<Term, f64> = HashMap::new();
        for term in vec![
            Term("This".to_owned()),
            Term("is".to_owned()),
            Term("a".to_owned()),
            Term("test".to_owned()),
            Term("sentence.".to_owned()),
        ] {
            expected.insert(term, 0.0);
        }

        for (term, idf) in &expected {
            assert_eq!(index.idf.get(term), Some(idf));
        }
    }
}
