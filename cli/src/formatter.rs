use ansi_term::Colour::{Blue, Yellow};

use indexer::{index_builder::file_processing::read_file_contents, search_query::SearchResult};
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use std::{
    collections::HashMap,
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
pub fn format_cli_output(results: Vec<SearchResult>) -> String {
    let mut output = String::new();
    for result in results.iter().take(10) {
        let path = encode_path(Path::new(&result.document.path));
        let relative_path = get_relative_path(&path);
        let formatted_path = Blue.bold().paint(relative_path).to_string();
        let get_line_matches = get_line_matches(&result.document.path, result.query_tokens.clone());
        let formatted_line_matches =
            format_line_match(get_line_matches, result.query_tokens.clone());
        output.push_str(&format!("\n{} [{:.2}]\n", formatted_path, result.score));
        // Print each match in formatted line matches
        for line in formatted_line_matches {
            output.push_str(&format!("{}\n", line));
        }
    }
    output
}

/// Print the lines of code that match the query
///
/// We have a Vec<Search Results> object that includes
/// a queried_tokens Vec<String>. We will open each file in
/// the top results and get the line of text that matches that token
/// we will highlight the specific word when it appears
///
/// Example output; where 25 is the line of text where it appears and
/// the rest is the actual content.
/// 25: This was the match
fn get_line_matches(path: &str, queried_tokens: Vec<String>) -> HashMap<usize, String> {
    let file_contents = read_file_contents(path).unwrap();
    let mut line_matches = HashMap::new();
    for (line_number, line) in file_contents.lines().enumerate() {
        for token in queried_tokens.iter() {
            // We want to check for all cases of the token:
            // e.g., Test, test, TEST
            // We will lowercase the line and the token
            let line = line.to_lowercase();
            let token = token.to_lowercase();
            if line.contains(&token) {
                line_matches.insert(line_number, line.to_string());
            }
        }
    }
    line_matches
}

/// Format line match
///
/// We want to make the line number green "70:"
/// and the rest of the text white.
///
/// We also want to highlight as dark blue the matches in the words.
fn format_line_match(
    line_matches: HashMap<usize, String>,
    queried_tokens: Vec<String>,
) -> Vec<String> {
    let mut formatted_line_matches = Vec::new();
    for (line_number, line) in line_matches {
        let mut formatted_line = String::new();
        let line_number = format!("{}:", line_number);
        let line_number = Blue.bold().paint(line_number).to_string();
        formatted_line.push_str(&line_number);
        formatted_line.push(' ');
        for token in queried_tokens.iter() {
            let bold_token = Yellow.bold().paint(token).to_string();
            let line = line.replace(token.as_str(), &format!("{}", bold_token));
            formatted_line.push_str(&line);
        }
        formatted_line_matches.push(formatted_line);
    }
    formatted_line_matches
}

/// Format the path with respect to the current paths
fn get_relative_path(path: &str) -> String {
    // We want to print the relative_path with
    // respect to the current working directory
    let current_wd = env::current_dir().unwrap();
    let full_path = Path::new(path);
    let relative_path = match full_path.strip_prefix(current_wd) {
        Ok(p) => p,
        Err(_) => full_path,
    };
    let file_url = format!("file://{}", path);
    let alias = Blue
        .bold()
        .paint(relative_path.display().to_string())
        .to_string();
    let hyperlink = format!("\x1B]8;;{}\x1B\\{}\x1B]8;;\x1B\\", file_url, alias);
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
    use std::collections::HashMap;

    use crate::formatter::format_line_match;

    #[test]
    fn test_relative_path() {
        let path = "/home/username/test.txt";
        let relative_path = super::get_relative_path(path);
        assert_eq!(relative_path, "test.txt");
    }

    #[test]
    fn test_format_line_match() {
        let mut line_matches: HashMap<usize, String> = HashMap::new();
        line_matches.insert(3, "This is a test sentence.".to_string());
        let queried_tokens = vec!["test".to_string()];
        let formatted_line_matches = format_line_match(line_matches, queried_tokens);
        let expected =
            vec!["\x1B[1;34m3:\x1B[0m This is a \x1B[1;34mtest\x1B[0m sentence.".to_string()];
        print!("{:?}", formatted_line_matches);
        assert_eq!(formatted_line_matches, expected);
    }
}
