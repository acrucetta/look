use std::collections::{HashMap, HashSet};

use crate::{
    data_ingestion::text_processing::process_text,
    indexer::{index_storage::Term, Index},
};

// Structure to store the document information and its relevance score
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SearchResult {
    pub document_path: String,
    pub score: f64,
}

impl SearchResult {
    // Function to create a new SearchResult
    pub fn new(document_path: String, score: f64) -> SearchResult {
        SearchResult {
            document_path,
            score,
        }
    }
}

pub fn search(query: &str, index: &Index) -> Vec<SearchResult> {
    // 1. Process the search query
    let processed_query = process_text(query);

    // 2. Calculate the query's TF-IDF
    let query_tfidf = calculate_query_tfidf(&processed_query, &index);

    // 3. Retrieve relevant documents
    let candidate_documents = retrieve_candidate_documents(&processed_query, &index);

    // 4. Rank the documents
    let ranked_documents = rank_documents(&candidate_documents, &query_tfidf, &index);

    // 5. Return the search results
    ranked_documents
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
fn calculate_query_tfidf(query: &str, index: &Index) -> HashMap<String, f64> {
    // Calculate the TF-IDF for each term in the query
    let mut query_tfidf = HashMap::new();
    let tokens = query.split_whitespace().collect::<Vec<&str>>();

    for token in tokens {
        let term = Term(token.to_owned());
        let idf = index.idf.get(&term).unwrap_or(&0.0);
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
fn retrieve_candidate_documents(query: &str, index: &Index) -> HashSet<String> {
    // Retrieve the candidate documents
    let mut candidate_documents = HashSet::new();
    let tokens = query.split_whitespace().collect::<Vec<&str>>();

    for token in tokens {
        let term = Term(token.to_owned());
        let docs = index.inverted_index.get(&term);
        match docs {
            Some(docs) => {
                for (doc, _) in docs {
                    candidate_documents.insert(doc.to_owned());
                }
            }
            None => {}
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
    candidate_documents: &HashSet<String>,
    query_tfidf: &HashMap<String, f64>,
    index: &Index,
) -> Vec<SearchResult> {
    let mut document_scores: HashMap<String, f64> = HashMap::new();

    for (term, query_tfidf_value) in query_tfidf.iter() {
        if let Some(document_frequencies) = index.inverted_index.get(&Term(term.to_string())) {
            for (document_path, term_frequency) in document_frequencies {
                if candidate_documents.contains(document_path) {
                    let idf = index.idf.get(&Term(term.to_string())).unwrap_or(&0.0);
                    let tf_idf = *term_frequency as f64 * idf;

                    let score = document_scores.entry(document_path.clone()).or_insert(0.0);
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
    let mut ranked_documents: Vec<SearchResult> = document_scores
        .iter()
        .map(|(document_path, score)| SearchResult::new(document_path.clone(), *score))
        .collect();

    ranked_documents.sort_unstable_by(|a, b| b.partial_cmp(&a).unwrap());

    ranked_documents
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        indexer::{index_storage::Term, Index},
        search_query::{
            query_processing::{rank_documents, retrieve_candidate_documents},
            SearchResult,
        },
    };

    fn create_sample_index() -> Index {
        let mut index = Index::new();

        index.inverted_index.insert(
            Term("apple".to_owned()),
            vec![("doc1.txt".to_owned(), 2), ("doc2.txt".to_owned(), 3)]
                .into_iter()
                .collect(),
        );
        index.inverted_index.insert(
            Term("banana".to_owned()),
            vec![("doc1.txt".to_owned(), 1), ("doc3.txt".to_owned(), 1)]
                .into_iter()
                .collect(),
        );
        index.idf.insert(Term("apple".to_owned()), 1.0);
        index.idf.insert(Term("banana".to_owned()), 1.0);

        index.document_norms.insert("doc1.txt".to_owned(), 2.236);
        index.document_norms.insert("doc2.txt".to_owned(), 3.0);
        index.document_norms.insert("doc3.txt".to_owned(), 1.0);

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
        assert_eq!(candidate_documents, expected_candidate_documents);
    }

    #[test]
    fn test_rank_documents() {
        let index = create_sample_index();
        let candidate_documents: HashSet<String> = [
            "doc1.txt".to_owned(),
            "doc2.txt".to_owned(),
            "doc3.txt".to_owned(),
        ]
        .iter()
        .cloned()
        .collect();
        let query_tfidf: HashMap<String, f64> =
            [("apple".to_owned(), 1.0), ("banana".to_owned(), 1.0)]
                .iter()
                .cloned()
                .collect();

        let expected_ranked_documents = vec![
            SearchResult::new("doc1.txt".to_owned(), 1.341),
            SearchResult::new("doc2.txt".to_owned(), 1.0),
            SearchResult::new("doc3.txt".to_owned(), 0.5),
        ];

        let ranked_documents = rank_documents(&candidate_documents, &query_tfidf, &index);
        assert_eq!(ranked_documents.len(), expected_ranked_documents.len());

        for (result, expected_result) in ranked_documents
            .iter()
            .zip(expected_ranked_documents.iter())
        {
            assert_eq!(result.document_path, expected_result.document_path);
            assert!((result.score - expected_result.score).abs() < 1e-3);
        }
    }
}
