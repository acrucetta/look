use ansi_term::Colour::Blue;
use indexer::search_query::SearchResult;
use std::path::{Path, PathBuf};

/// Format the CLI output
///
/// This function will format the CLI output
///
/// The final format will be:
///
/// Search results
/// ----------------
/// Doc: `test.txt` [0.50]
/// Doc: `test2.txt` [0.25]
///
/// The paths of the documents will be clickable, we will shorten them
/// to 20 characters and add an ellipsis if they are longer than 20 characters
/// they will also be blue ansi.
fn format_cli_output(results: Vec<SearchResult>) -> String {
    let mut output = String::new();
    output.push_str("Search results\n");
    output.push_str("----------------\n");
    for result in results.iter().take(10) {
        let path = shorten_path(Path::new(&result.document.path));
        output.push_str(&format!(
            "Doc: `{}` [{}]\n",
            Blue.bold().paint(path.to_str().unwrap()),
            result.score
        ));
    }
    output
}

fn shorten_path(path: &Path) -> PathBuf {
    let mut shortened_path = PathBuf::new();
    if let Ok(path_after_prefix) = path.strip_prefix("/") {
        shortened_path.push("...");
        shortened_path.push(path_after_prefix);
    } else {
        shortened_path.push(path);
    }
    shortened_path
}
