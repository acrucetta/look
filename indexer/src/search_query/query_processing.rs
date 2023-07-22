use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::index_builder::{Document, Index, Term};

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
        let tf_idf = calculate_query_tfidf(query, index);
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
    let candidate_documents = retrieve_candidate_documents(&query, index);
    let mut ranked_documents = rank_documents(&candidate_documents, &query.tf_idf, index);
    // Sort the documents by score in descending order
    ranked_documents.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    // Add the query tokens to the ranked documents structure
    ranked_documents.iter_mut().for_each(|doc| {
        doc.query_tokens = query.tokens.clone();
    });
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
    use super::*;

    #[test]
    fn test_tokenize_query() {
        let query = "simple query";
        let tokens = tokenize_query(query);
        assert_eq!(tokens, vec!["simple", "query"]);
    }

    #[test]
    fn test_calculate_query_tfidf() {
        let index = Index::new();
        let query = "this is a query";
        let query_tfidf = calculate_query_tfidf(query, &index);
        let expected_tfidf: HashMap<String, f64> = [
            ("this".to_owned(), 1.0),
            ("is".to_owned(), 1.0),
            ("a".to_owned(), 1.0),
            ("query".to_owned(), 1.0),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(query_tfidf, expected_tfidf);
    }

    #[test]
    fn test_retrieve_candidate_documents() {
        let mut index = Index::new();
        let document1 = Document {
            path: "doc1.txt".to_owned(),
        };
        let document2 = Document {
            path: "doc2.txt".to_owned(),
        };
        index.store_processed_text_in_index(&document1, "this is a sample document");
        index.store_processed_text_in_index(&document2, "another example document");
        index.calculate_idf();

        let query = Query::new("sample", &index);
        let candidate_documents = retrieve_candidate_documents(&query, &index);

        let expected_candidates: HashSet<Document> = [document1].iter().cloned().collect();
        assert_eq!(candidate_documents, expected_candidates);
    }

    #[test]
    fn test_rank_documents() {
        let mut index = Index::new();
        let document1 = Document {
            path: "doc1.txt".to_owned(),
        };
        let document2 = Document {
            path: "doc2.txt".to_owned(),
        };
        index.store_processed_text_in_index(&document1, "this is a sample document");
        index.store_processed_text_in_index(&document2, "another example document");
        index.calculate_idf();

        let query = Query::new("sample document", &index);
        let candidate_documents = retrieve_candidate_documents(&query, &index);

        let ranked_docs = rank_documents(&candidate_documents, &query.tf_idf, &index);

        // The first document should have a score of 1.0, and the second document s
        // hould have a score of 0.0. We will just try to be close to these values
        let expected_ranked_docs = vec![
            SearchResult::new(document1.clone(), 1.0),
            SearchResult::new(document2.clone(), 0.0),
        ];
        assert_eq!(ranked_docs, expected_ranked_docs);
    }

    #[test]
    fn test_search() {
        let mut index = Index::new();
        let document1 = Document {
            path: "doc1.txt".to_owned(),
        };
        let document2 = Document {
            path: "doc2.txt".to_owned(),
        };
        index.store_processed_text_in_index(&document1, "this is a sample document");
        index.store_processed_text_in_index(&document2, "another example document");
        index.calculate_idf();

        let query = "sample document";
        let search_results = search(query, &index).unwrap();

        let expected_search_results = vec![
            SearchResult::new(document1.clone(), 1.0),
            SearchResult::new(document2.clone(), 0.0),
        ];
        assert_eq!(search_results, expected_search_results);
    }

    #[test]
    fn test_sort_vectors() {
        let mut results = vec![
            SearchResult {
                document: Document {
                    path: "path1.txt".to_owned(),
                },
                score: 0.5,
                query_tokens: todo!(),
                matched_lines: todo!(),
                file_type: todo!(),
            },
            SearchResult {
                document: Document {
                    path: "path2.txt".to_owned(),
                },
                score: 0.2,
                query_tokens: todo!(),
                matched_lines: todo!(),
                file_type: todo!(),
            },
            SearchResult {
                document: Document {
                    path: "path3".to_owned(),
                },
                score: 0.8,
                query_tokens: todo!(),
                matched_lines: todo!(),
                file_type: todo!(),
            },
        ];
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        println!("Sorted results: {:?}", results);

        assert!(results[0].score > results[1].score);
    }
}
