use ansi_term::Colour::Blue;
use indexer::search_query::SearchResult;
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use std::{
    env,
    path::{Path, PathBuf},
};

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
        let relative_path = get_relative_path(path);
        let formatted_path = Blue.bold().paint(relative_path).to_string();
        output.push_str(&format!("\n{} [{:.2}]\n", formatted_path, result.score))
    }
    output
}

/// Format the path with respect to the current paths
fn get_relative_path(path: String) -> String {
    // We want to print the relative_path with
    // respect to the current working directory
    let current_wd = env::current_dir().unwrap();
    let full_path = Path::new(&path);
    let relative_path = match full_path.strip_prefix(current_wd) {
        Ok(p) => p,
        Err(_) => full_path,
    };
    let hyperlink = format!(
        "\u{1b}[38;5;81;4mfile://{}\u{1b}[0m",
        relative_path.display()
    );
    hyperlink
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

#[cfg(test)]
mod tests {

    #[test]
    fn test_relative_path() {
        let path = "/home/username/test.txt";
        let relative_path = super::get_relative_path(path.to_string());
        assert_eq!(relative_path, "test.txt");
    }
}
