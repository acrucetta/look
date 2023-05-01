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
                    .map(|(doc, freq)| (doc.clone(), freq.clone()))
                    .collect(),
            )
        })
        .collect()
}

pub fn deserialize_hashmap_from_vec<T: Clone, U: Clone>(
    vec: &Vec<(T, U)>,
) -> std::collections::HashMap<T, U> {
    vec.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<HashMap<T, U>>()
}

pub fn deserialize_inverted_index(
    vec: &Vec<(Term, Vec<(Document, u32)>)>,
) -> std::collections::HashMap<Term, HashMap<Document, u32>> {
    vec.iter()
        .map(|(term, docs)| {
            (
                term.clone(),
                docs.iter()
                    .map(|(doc, freq)| (doc.clone(), freq.clone()))
                    .collect(),
            )
        })
        .collect::<HashMap<Term, HashMap<Document, u32>>>()
}
