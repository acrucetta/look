use crate::data_ingestion::text_processing::process_text;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use super::IndexStorage;

pub struct FileProcessing;

pub fn process_directory<P: AsRef<Path>>(
    path: P,
    index: &mut IndexStorage,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                process_file(&path, index)?;
            } else if path.is_dir() {
                process_directory(&path, index)?;
            }
        }
    } else {
        return Err(From::from("Input path must be a directory."));
    }

    Ok(())
}

pub fn process_file<P: AsRef<Path>>(
    path: P,
    index: &mut IndexStorage,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Apply your text processing function here
    let processed_text = process_text(&content);

    // Store the processed text in the index (e.g., in a database or an inverted index)
    index.store_processed_text_in_index(&processed_text, path);

    Ok(())
}
