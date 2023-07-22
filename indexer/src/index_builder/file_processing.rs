use crate::data_ingestion;
use crate::data_ingestion::text_processing::process_text;
use data_ingestion::file_handler::*;
use std::fs;
use std::path::Path;

use super::{Document, Index};

pub fn process_directory<P: AsRef<Path>>(
    path: P,
    index: &mut Index,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                // We will skip files that return an error when we try to process them.
                if let Err(e) = process_file(&path, index) {
                    println!("Error processing file: {}", e);
                }
            } else if path.is_dir() {
                process_directory(&path, index)?;
            }
        }
    } else {
        return Err(From::from("Input path must be a directory."));
    }

    Ok(())
}

pub fn get_file_extension<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    let path = path.as_ref();
    let file_extension = match {
        let extension = path.extension();
        extension.and_then(|s| s.to_str())
    } {
        Some(extension) => extension,
        // If the file has no extension, we'll skip it with a warning.
        None => return Err(From::from("File has no extension.")),
    };

    Ok(file_extension.to_owned())
}

pub fn read_file_contents<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    let path = path.as_ref();
    let file_extension = get_file_extension(&path)?;

    match file_extension.as_str() {
        "md" => {
            let file_handler = data_ingestion::MarkdownHandler;
            let content = file_handler.read_contents(path.to_str().unwrap())?;
            Ok(content)
        }
        "txt" => {
            let file_handler = data_ingestion::PlainTextHandler;
            let content = file_handler.read_contents(path.to_str().unwrap())?;
            Ok(content)
        }
        _ => Err(From::from(format!(
            "File extension {} is not supported.",
            file_extension
        ))),
    }
}

pub fn process_file<P: AsRef<Path>>(
    path: P,
    index: &mut Index,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();
    let content = read_file_contents(&path)?;
    let document = Document::new(path.to_str().unwrap().to_owned());
    Ok(index.store_processed_text_in_index(&document, &content))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_process_file() {
        let file_path = "../data/lorem_ipsum.txt";
        let mut index = super::Index::new();
        super::process_file(file_path, &mut index).unwrap();
        print!("{:?}", index.inverted_index);
        assert_eq!(index.inverted_index.len(), 69);
    }

    #[test]
    fn test_process_directory() {
        let dir_path = "data";
        let mut index = super::Index::new();
        super::process_directory(dir_path, &mut index).unwrap();
        print!("{:?}", index.inverted_index);
        assert_eq!(index.inverted_index.len(), 3);
    }
}
