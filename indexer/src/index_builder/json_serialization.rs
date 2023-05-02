use std::collections::HashMap;

use super::{Document, Term};

pub fn serialize_hashmap_to_vec<T: Clone, U: Clone>(
    hashmap: &std::collections::HashMap<T, U>,
) -> Vec<(T, U)> {
    hashmap
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

pub fn serialize_inverted_index(
    inverted_index: &std::collections::HashMap<Term, HashMap<Document, u32>>,
) -> Vec<(Term, Vec<(Document, u32)>)> {
    inverted_index
        .iter()
        .map(|(term, docs)| {
            (
                term.clone(),
                docs.iter()
                    .map(|(doc, freq)| (doc.clone(), *freq))
                    .collect(),
            )
        })
        .collect()
}

pub fn deserialize_vec_to_hashmap<T: Clone + Eq + std::hash::Hash, U: Clone>(
    vec: &[(T, U)],
) -> HashMap<T, U> {
    vec.iter().cloned().collect()
}

pub fn deserialize_inverted_index(
    vec: &[(Term, Vec<(Document, u32)>)],
) -> HashMap<Term, HashMap<Document, u32>> {
    vec.iter()
        .map(|(term, docs)| (term.clone(), deserialize_vec_to_hashmap(docs)))
        .collect()
}
