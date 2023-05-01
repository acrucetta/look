use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::{Document, Term};

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub inverted_index: HashMap<Term, HashMap<Document, u32>>,
    pub idf: HashMap<Term, f64>,
    pub document_norms: HashMap<Document, f64>,
    pub num_docs: usize,
}

impl Index {
    pub fn new() -> Index {
        Index {
            inverted_index: HashMap::new(),
            idf: HashMap::new(),
            document_norms: HashMap::new(),
            num_docs: 0,
        }
    }

    pub fn store_processed_text_in_index(&mut self, document: &Document, text: &str) {
        let tokens = text.split_whitespace().collect::<Vec<&str>>();
        self.num_docs += 1;

        for token in tokens {
            self.insert_token(token, document);
        }

        self.update_document_norm(document);
    }

    /// Function to calculate the IDF for each term in the index
    ///
    /// # Arguments
    /// * `self` - The index to calculate the IDF for
    ///  
    /// # Returns
    /// * `()` - The function returns nothing
    ///
    /// An example of what IDF does is shown below:
    ///
    pub fn calculate_idf(&mut self) {
        for (term, docs) in &self.inverted_index {
            let idf = self.num_docs as f64 / docs.len() as f64;
            self.idf.insert(term.clone(), idf);
        }
    }

    /// Function to load the index from a JSON file
    ///
    /// # Arguments
    ///  * `index_path` - The path to the JSON file
    ///  
    /// # Returns
    ///  * `std::io::Result<Index>` - The index
    pub fn load_index_from_json_file(index_path: &Path) -> std::io::Result<Index> {
        use super::json_serialization::deserialize_hashmap_from_vec;

        let mut file = File::open(index_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let index: Index = serde_json::from_str(&contents)?;

        let inverted_index = deserialize_hashmap_from_vec(&index.inverted_index);
        let idf = deserialize_hashmap_from_vec(&index.idf);
        let document_norms = deserialize_hashmap_from_vec(&index.document_norms);

        Ok(Index {
            inverted_index,
            idf,
            document_norms,
            num_docs: index.num_docs,
        })
    }

    /// Function to save the index to a JSON file
    pub fn save_index_to_json_file(&self, output_path: &Path) -> std::io::Result<()> {
        use super::json_serialization::{serialize_hashmap_to_vec, serialize_inverted_index};

        let inverted_index = serialize_inverted_index(&self.inverted_index);
        let idf = serialize_hashmap_to_vec(&self.idf);
        let document_norms = serialize_hashmap_to_vec(&self.document_norms);

        let index = serde_json::json!({
                "inverted_index": inverted_index,
                "idf": idf,
                "document_norms": document_norms,
                "num_docs": self.num_docs,
        });

        let mut file = File::create(output_path)?;
        file.write_all(index.to_string().as_bytes())?;

        Ok(())
    }

    fn insert_token(&mut self, token: &str, document: &Document) {
        let term = Term(token.to_owned());
        let entry = self
            .inverted_index
            .entry(term.clone())
            .or_insert_with(HashMap::new);
        let term_frequency = entry.entry(document.clone()).or_insert(0);
        *term_frequency += 1;
    }

    fn update_document_norm(&mut self, document: &Document) {
        let mut document_norm = 0.0;
        for (_, term_frequency) in self.inverted_index.values().flatten() {
            document_norm += (*term_frequency as f64).powi(2);
        }
        document_norm = document_norm.sqrt();
        self.document_norms.insert(document.clone(), document_norm);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::index_builder::index_storage::Term;

    fn build_index_with_3_docs() -> super::Index {
        use super::Index;
        use std::path::Path;

        let mut index = Index::new();
        let text = "This is a test sentence.";
        let path = Path::new("test.txt");
        let document = super::Document::new(path.to_str().unwrap().to_owned());
        index.store_processed_text_in_index(&document, &text);

        let text = "This is another test sentence.";
        let path = Path::new("test2.txt");
        let document = super::Document::new(path.to_str().unwrap().to_owned());
        index.store_processed_text_in_index(&document, &text);

        let text = "This is a third test sentence.";
        let path = Path::new("test3.txt");
        let document = super::Document::new(path.to_str().unwrap().to_owned());
        index.store_processed_text_in_index(&document, &text);

        index
    }

    #[test]
    fn test_store_processed_text_in_index() {
        use super::Index;
        use std::path::Path;

        let mut index = Index::new();
        let text = "This is a test sentence.";
        let path = Path::new("test.txt");
        let document = super::Document::new(path.to_str().unwrap().to_owned());

        index.calculate_idf();
        index.store_processed_text_in_index(&document, &text);

        assert_eq!(index.num_docs, 1);
        assert_eq!(index.inverted_index.len(), 5);

        // Create a HashMap of the expected results
        let mut expected: HashMap<Term, HashMap<super::Document, u32>> = HashMap::new();

        for term in vec![
            Term("This".to_owned()),
            Term("is".to_owned()),
            Term("a".to_owned()),
            Term("test".to_owned()),
            Term("sentence.".to_owned()),
        ] {
            let mut documents = HashMap::new();
            documents.insert(document.clone(), 1);
            expected.insert(term, documents);
        }

        for (term, docs) in &expected {
            assert_eq!(index.inverted_index.get(term), Some(docs));
        }
    }

    #[test]
    fn test_calculate_idf() {
        // Create an index with 3 documents
        let mut index = build_index_with_3_docs();
        index.calculate_idf();

        // Calculate the IDF for each term
        let mut expected: HashMap<Term, f64> = HashMap::new();

        for term in vec![
            Term("This".to_owned()),
            Term("is".to_owned()),
            Term("a".to_owned()),
            Term("test".to_owned()),
            Term("sentence.".to_owned()),
            Term("another".to_owned()),
            Term("third".to_owned()),
        ] {
            let idf = 3.0 / index.inverted_index.get(&term).unwrap().len() as f64;
            expected.insert(term, idf);
        }

        for (term, idf) in &expected {
            assert_eq!(index.idf.get(term), Some(idf));
        }
    }
}
