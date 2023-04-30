mod data_ingestion;
mod indexer;
use indexer::file_processing::process_directory;
use indexer::Index;
use std::path::Path;

fn main() {
    let dir_path = "data";
    let mut index = Index::new();

    // Update your `process_file` function to accept a `&mut HashMap<String, Vec<String>>` argument and pass it to `store_processed_text_in_index`
    match process_directory(dir_path, &mut index) {
        Ok(_) => println!("Processing completed."),
        Err(e) => println!("Error occurred: {}", e),
    }

    // Calculate the IDF for each term
    index.calculate_idf();

    // Save the in-memory index to a JSON file
    let output_path = Path::new("index.json");
    match index.save_index_to_json_file(output_path) {
        Ok(_) => println!("Index saved to {}.", output_path.to_str().unwrap()),
        Err(e) => println!("Error occurred: {}", e),
    }
}
