use ansi_term::Colour::Blue;
use indexer::search_query::SearchResult;
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use std::path::{Path, PathBuf};

// Add the characters we want to exclude from percent encoding
const ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'#').add(b'?');

/// Format the CLI output
///
/// This function will format the CLI output
///
/// The final format will be:
///
/// Search results
/// ----------------
/// Doc: file:///home/username/test.txt [0.5]
/// Doc: file:///home/username/test.txt [0.5]
///
/// The paths of the documents will be clickable, we will shorten them
/// to 20 characters and add an ellipsis if they are longer than 20 characters
/// they will also be blue ansi.
pub(crate) fn format_cli_output(results: Vec<SearchResult>) -> String {
    let mut output = String::new();
    output.push_str("Search results\n");
    output.push_str("----------------\n");
    for result in results.iter().take(10) {
        let path = encode_path(Path::new(&result.document.path));
        let formatted_path = Blue.bold().paint(path).to_string();
        output.push_str(&format!(
            "Doc: {}{} [{:.2}]\n",
            "file://", formatted_path, result.score
        ));
    }
    output
}

fn encode_path(path: &Path) -> String {
    let mut path_buf = PathBuf::new();
    for component in path.components() {
        let encoded_component = percent_encode(
            component.as_os_str().to_str().unwrap().as_bytes(),
            ENCODE_SET,
        )
        .to_string();
        path_buf.push(encoded_component);
    }
    path_buf.to_str().unwrap().to_string()
}
