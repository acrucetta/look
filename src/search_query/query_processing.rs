use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::{
    data_ingestion::text_processing::process_text,
    indexer::{file_processing::Document, index_storage::Term, Index},
};

use super::SearchResult;

pub struct Query {
    pub raw: String,
    pub tokens: Vec<String>,
    pub tf_idf: HashMap<String, f64>,
}

fn tokenize_query(query: &str) -> Vec<String> {
    query
        .split_whitespace()
        .map(|token| token.to_owned())
        .collect()
}

impl Query {
    pub fn new(query: &str, index: &Index) -> Self {
        let tokens = tokenize_query(query);
        let tf_idf = calculate_query_tfidf(&tokens, index);
        Query {
            raw: query.to_owned(),
            tokens,
            tf_idf,
        }
    }
}

// Structure to store the document information and its relevance score

pub fn search(query: &str, index: &Index) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    let query = Query::new(query, index);
    let candidate_documents = retrieve_candidate_documents(&query, index)?;
    let ranked_documents = rank_documents(&candidate_documents, &query, index)?;
    Ok(ranked_documents)
}

/// Function to calculate the query's TF-IDF
///
/// # Arguments
///  * `query` - The query to calculate the TF-IDF for
///  * `index` - The index to use to calculate the TF-IDF
///
/// # Returns
///  * A HashMap containing the TF-IDF for each term in the query
///
pub fn calculate_query_tfidf(query: &str, index: &Index) -> HashMap<String, f64> {
    // Calculate the TF-IDF for each term in the query
    let mut query_tfidf = HashMap::new();
    let tokens = query.split_whitespace().collect::<Vec<&str>>();

    for token in tokens {
        let term = Term(token.to_owned());
        let idf = index.idf.get(&term).unwrap_or(&1.0);
        let count = query_tfidf.entry(token.to_owned()).or_insert(0.0);
        *count += 1.0 * idf;
    }

    query_tfidf
}

/// Function to retrieve the candidate documents for the query
///
/// # Arguments
///  * `query` - The query to retrieve the candidate documents for
///  * `index` - The index to use to retrieve the candidate documents
///
/// # Returns
///  * A HashSet containing the paths of the candidate documents
pub fn retrieve_candidate_documents(query: &Query, index: &Index) -> HashSet<Document> {
    let mut candidate_documents = HashSet::new();
    for token in &query.tokens {
        let term = Term(token.to_owned());
        let docs = index.inverted_index.get(&term);
        if let Some(docs) = docs {
            for (doc, _) in docs {
                candidate_documents.insert(doc.to_owned());
            }
        }
    }
    candidate_documents
}

/// Function to rank the candidate documents
///
/// # Arguments
///  * `candidate_documents` - The candidate documents to rank
///  * `query_tfidf` - The TF-IDF for the query
///  * `index` - The index to use to rank the documents
///
/// # Returns
///  * A vector of `SearchResult`s containing the ranked documents
fn rank_documents(
    candidate_documents: &HashSet<Document>,
    query_tfidf: &HashMap<String, f64>,
    index: &Index,
) -> Vec<SearchResult> {
    let mut document_scores: HashMap<Document, f64> = HashMap::new();

    for (term, query_tfidf_value) in query_tfidf.iter() {
        if let Some(document_frequencies) = index.inverted_index.get(&Term(term.to_string())) {
            for (document_path, term_frequency) in document_frequencies {
                if candidate_documents.contains(document_path) {
                    let idf = index.idf.get(&Term(term.to_string())).unwrap_or(&1.0);
                    let tf_idf = *term_frequency as f64 * idf;

                    let score = document_scores
                        .entry(document_path.to_owned())
                        .or_insert(0.0);
                    *score += query_tfidf_value * tf_idf;
                }
            }
        }
    }

    // Normalize the scores
    for (document_path, score) in document_scores.iter_mut() {
        let document_norm = index.document_norms.get(document_path).unwrap_or(&1.0);
        let query_norm = query_tfidf
            .values()
            .map(|value| value * value)
            .sum::<f64>()
            .sqrt();

        *score /= document_norm * query_norm;
    }

    // Sort the documents by score in descending order
    let ranked_docs = document_scores
        .iter()
        .map(|(document_path, score)| SearchResult::new(document_path.clone(), *score))
        .collect::<Vec<SearchResult>>();

    ranked_docs
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        indexer::{file_processing::Document, Index},
        search_query::{
            query_processing::{rank_documents, retrieve_candidate_documents},
            SearchResult,
        },
    };

    /// Function to create a sample index
    ///
    /// # Returns
    /// * A sample index
    ///
    /// The sample index contains the following:
    /// * Inverted index:
    ///    * Term: apple
    ///       * Document: doc1.txt
    ///         * Term frequency: 2
    ///      * Document: doc2.txt
    ///        * Term frequency: 3
    ///   * Term: banana
    ///     * Document: doc1.txt
    ///      * Term frequency: 1
    ///    * Document: doc3.txt
    ///     * Term frequency: 1
    /// * IDF:
    ///  * Term: apple
    ///  * IDF: 1.0
    /// * Term: banana
    /// * IDF: 1.0
    /// * Document norms:
    /// * Document: doc1.txt
    /// * Norm: 2.236
    /// * Document: doc2.txt
    /// * Norm: 3.0
    /// * Document: doc3.txt
    /// * Norm: 1.0
    fn create_sample_index() -> Index {
        let mut index = Index::new();

        let doc1 = Document::new("doc1.txt".to_owned(), "apple apple banana".to_owned());
        let doc2 = Document::new(
            "doc2.txt".to_owned(),
            "apple apple apple banana banana".to_owned(),
        );
        let doc3 = Document::new("doc3.txt".to_owned(), "banana".to_owned());

        index.store_processed_text_in_index(&doc1);
        index.store_processed_text_in_index(&doc2);
        index.store_processed_text_in_index(&doc3);

        index.calculate_idf();

        index
    }

    #[test]
    fn test_calculate_query_idf() {
        let index = create_sample_index();

        let query = "apple banana";
        let query_tfidf = super::calculate_query_tfidf(query, &index);

        assert_eq!(query_tfidf.len(), 2);
    }

    #[test]
    fn test_getting_document_score() {
        // Create a sample index
        let index = create_sample_index();

        // Create a sample query
        let query = "apple banana";

        // Calculate the query's TF-IDF
        let query_tfidf = super::calculate_query_tfidf(query, &index);

        // Retrieve the candidate documents
        let candidate_documents = retrieve_candidate_documents(query, &index);

        // Rank the documents
        let ranked_documents = rank_documents(&candidate_documents, &query_tfidf, &index);

        // Check the results
        assert_eq!(ranked_documents.len(), 3);
        assert_eq!(ranked_documents[0].document.path, "doc1.txt");
        assert!((ranked_documents[0].score - 1.341).abs() < 1e-3);
    }

    #[test]
    fn test_retrieve_candidate_documents() {
        let index = create_sample_index();
        let query = "apple banana";
        let expected_candidate_documents: HashSet<String> = [
            "doc1.txt".to_owned(),
            "doc2.txt".to_owned(),
            "doc3.txt".to_owned(),
        ]
        .iter()
        .cloned()
        .collect();

        let candidate_documents = retrieve_candidate_documents(query, &index);
        assert_eq!(
            candidate_documents.len(),
            expected_candidate_documents.len()
        );
    }

    #[test]
    fn test_rank_documents() {
        let index = create_sample_index();

        let candidate_documents: HashSet<Document> = [
            Document::new("doc1.txt".to_owned(), "apple apple banana".to_owned()),
            Document::new(
                "doc2.txt".to_owned(),
                "apple apple apple banana banana".to_owned(),
            ),
            Document::new("doc3.txt".to_owned(), "banana".to_owned()),
        ]
        .iter()
        .cloned()
        .collect();

        let query_tfidf: HashMap<String, f64> =
            [("apple".to_owned(), 1.0), ("banana".to_owned(), 1.0)]
                .iter()
                .cloned()
                .collect();

        let expected_ranked_documents: Vec<SearchResult> = vec![
            SearchResult::new(
                Document::new("doc1.txt".to_owned(), "apple apple banana".to_owned()),
                1.3416407864998738,
            ),
            SearchResult::new(
                Document::new(
                    "doc2.txt".to_owned(),
                    "apple apple apple banana banana".to_owned(),
                ),
                0.8944271909999159,
            ),
            SearchResult::new(
                Document::new("doc3.txt".to_owned(), "banana".to_owned()),
                0.0,
            ),
        ];

        let ranked_documents = rank_documents(&candidate_documents, &query_tfidf, &index);
        assert_eq!(ranked_documents.len(), expected_ranked_documents.len());

        for (result, expected_result) in ranked_documents
            .iter()
            .zip(expected_ranked_documents.iter())
        {
            assert_eq!(result.document, expected_result.document);
            assert!((result.score - expected_result.score).abs() < 1e-3);
        }
    }
}
