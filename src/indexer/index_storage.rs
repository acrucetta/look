use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::file_processing::Document;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct Term(pub String);

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

    pub fn store_processed_text_in_index(&mut self, document: &Document) {
        // Tokenize the processed text
        let tokens = document.contents.split_whitespace().collect::<Vec<&str>>();

        // Increment the number of documents in the index
        self.num_docs += 1;

        // Insert each token into the index and calculate document norms
        let mut document_tfidf_squared_sum: f64 = 0.0;
        for token in tokens {
            let term = Term(token.to_owned());
            let entry = self
                .inverted_index
                .entry(term.clone())
                .or_insert_with(HashMap::new);
            let term_frequency = entry.entry(document.clone()).or_insert(0);
            *term_frequency += 1;

            let idf = self.idf.get(&term).unwrap_or(&1.0);
            let tf_idf = *term_frequency as f64 * idf;
            document_tfidf_squared_sum += tf_idf * tf_idf;
        }

        let document_norm = document_tfidf_squared_sum.sqrt();
        self.document_norms.insert(document.clone(), document_norm);
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

    fn build_index_with_3_docs() -> super::Index {
        use super::Index;
        use std::path::Path;

        let mut index = Index::new();
        let text = "This is a test sentence.";
        let path = Path::new("test.txt");
        let document = super::Document::new(path.to_str().unwrap().to_owned(), text.to_owned());
        index.store_processed_text_in_index(&document);

        let text = "This is another test sentence.";
        let path = Path::new("test2.txt");
        let document = super::Document::new(path.to_str().unwrap().to_owned(), text.to_owned());
        index.store_processed_text_in_index(&document);

        let text = "This is a third test sentence.";
        let path = Path::new("test3.txt");
        let document = super::Document::new(path.to_str().unwrap().to_owned(), text.to_owned());
        index.store_processed_text_in_index(&document);

        index
    }

    #[test]
    fn test_store_processed_text_in_index() {
        use super::Index;
        use std::path::Path;

        let mut index = Index::new();
        let text = "This is a test sentence.";
        let path = Path::new("test.txt");
        let document = super::Document::new(text.to_owned(), path.to_str().unwrap().to_owned());
        index.store_processed_text_in_index(&document);

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
