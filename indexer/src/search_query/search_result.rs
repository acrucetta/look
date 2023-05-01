use crate::index_builder::Document;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SearchResult {
    pub document: Document,
    pub score: f64,
}

impl SearchResult {
    // Function to create a new SearchResult
    pub fn new(document_path: Document, score: f64) -> SearchResult {
        SearchResult {
            document: document_path,
            score,
        }
    }
}

impl std::fmt::Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({})", self.document.path, self.score)
    }
}
