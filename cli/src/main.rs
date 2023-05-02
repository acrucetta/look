use clap::{arg, command, Command};
use indexer::index_builder::Index;
use indexer::search_query;
use indexer::search_query::SearchResult;
use std::env;
use std::fs::File;
use std::io::Read;
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

    let index_path = env::var("INDEX_PATH").expect("INDEX_PATH not set");
    let index = load_index_from_json_file(Path::new(&index_path)).expect("Failed to load index");

    match matches.subcommand() {
        Some(("fd", matches)) => {
            let query = matches.get_one::<String>("QUERY").unwrap();
            search(query, index);
        }
        Some(("reindex", matches)) => {
            reindex();
        }
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
            for result in results {
                println!("{}", result);
            }
        }
        Err(e) => println!("Error occurred: {}", e),
    }
    search_results
}

/// Re-index a directory
///
/// This function will re-index a directory and save the index to the INDEX_PATH
fn reindex() {
    todo!()
}

/// Load an index from a JSON file
///
/// This function will load an index from a JSON file
///
/// # Arguments
///  * `index_path` - The path to the JSON file
///
/// # Returns
/// * `std::io::Result<Index>` - The index
fn load_index_from_json_file(index_path: &Path) -> std::io::Result<Index> {
    let mut file = File::open(index_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let index: Index = serde_json::from_str(&contents)?;
    Ok(index)
}
