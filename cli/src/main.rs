use clap::{arg, command, Command};
use indexer::index_builder::file_processing::process_directory;
use indexer::index_builder::Index;
use indexer::search_query;
use indexer::search_query::SearchResult;
use std::env;
use std::path::Path;

fn main() {
    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("fd")
                .about("Find a document with a query")
                .arg(arg!([QUERY]))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("reindex")
                .about("Re-index a directory")
                .arg_required_else_help(true),
        )
        .get_matches();

    let index_path = "index.json";
    let index =
        Index::load_index_from_json_file(Path::new(&index_path)).expect("Failed to load index");

    match matches.subcommand() {
        Some(("fd", matches)) => {
            let query = matches.get_one::<String>("QUERY").unwrap();
            search(query, index);
        }
        Some(("reindex", _matches)) => {
            reindex();
        }
        _ => unreachable!(),
    }
}

/// Search for a query in an index
///
/// This function will search for a query in an index
///
/// # Arguments
///  * `query` - The query to search for
///  * `index` - The index to search in
///
/// # Returns
///  * `Vec<String>` - The results of the search
fn search(query: &String, index: Index) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let search_results = search_query::search(&query, &index);
    // Print the search results
    println!("Search results:");
    match search_results {
        Ok(results) => {
            for result in &results {
                println!("{}", result);
            }
            Ok(results)
        }
        Err(e) => Err(e),
    }
}

/// Re-index a directory
///
/// This function will re-index a directory and save the index to the INDEX_PATH
fn reindex() {
    let dir_path = "personal_data";
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

#[cfg(test)]
mod tests {
    use std::{env, path::Path};

    fn build_index_with_3_docs() -> super::Index {
        use super::Index;
        use indexer::index_builder::Document;

        let mut index = Index::new();
        let text = "This is a test sentence.";
        let path = Path::new("test.txt");
        let document = Document::new(path.to_str().unwrap().to_owned());
        index.store_processed_text_in_index(&document, &text);

        let text = "This is another test sentence.";
        let path = Path::new("test2.txt");
        let document = Document::new(path.to_str().unwrap().to_owned());
        index.store_processed_text_in_index(&document, &text);

        let text = "This is a third test sentence.";
        let path = Path::new("test3.txt");
        let document = Document::new(path.to_str().unwrap().to_owned());
        index.store_processed_text_in_index(&document, &text);

        // Save the index to a JSON file
        index.calculate_idf();
        index
            .save_index_to_json_file(&Path::new("test_index.json"))
            .unwrap();
        index
    }

    #[test]
    fn test_search() {
        let index = build_index_with_3_docs();

        let query = "test";
        let results = super::search(&query.to_owned(), index).unwrap();

        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_reindex() {
        // Set the current directory to the root of the project
        let root_dir = env::current_dir().unwrap();

        let mut index_path = root_dir.clone();
        index_path.push("index.json");

        let mut personal_data_path = root_dir.clone();
        personal_data_path.push("personal_data");

        // Delete the index file if it exists
        if index_path.exists() {
            std::fs::remove_file(index_path.clone()).unwrap();
        }

        // Re-index the directory
        super::reindex();

        // Check that the index file was created
        assert!(index_path.exists());

        // Delete the index file
        std::fs::remove_file(index_path.clone()).unwrap();
    }
}
