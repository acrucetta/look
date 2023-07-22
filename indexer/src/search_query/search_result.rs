use crate::index_builder::Document;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SearchResult {
    pub document: Document,
    pub score: f64,
    pub query_tokens: Vec<String>,
    pub matched_lines: Vec<String>,
    pub file_type: String,
}

impl SearchResult {
    // Function to create a new SearchResult
    pub fn new(document_path: Document, score: f64) -> SearchResult {
        SearchResult {
            document: document_path,
            score,
            query_tokens: Vec::new(),
            matched_lines: Vec::new(),
            file_type: String::new(),
        }
    }
}

impl std::fmt::Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({})", self.document.path, self.score)
    }
}
