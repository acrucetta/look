use clap::{App, Arg};
use indexer::{search, Index};
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let matches = App::new("CLI Search")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Searches in the indexed documents")
        .arg(
            Arg::new("query")
                .about("The query to search for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("index_path")
                .short('i')
                .long("index")
                .value_name("INDEX_PATH")
                .about("The path to the index.json file")
                .takes_value(true)
                .default_value("index.json"),
        )
        .get_matches();

    let query = matches.value_of("query").unwrap();
    let index_path = matches.value_of("index_path").unwrap();

    let index = load_index_from_json_file(Path::new(index_path)).expect("Failed to load index");
    let results = search(query, &index).expect("Search failed");

    for result in results {
        println!("{}", result.document_path);
    }
}

fn load_index_from_json_file(index_path: &Path) -> std::io::Result<Index> {
    let mut file = File::open(index_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let index: Index = serde_json::from_str(&contents)?;
    Ok(index)
}
