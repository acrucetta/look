use rust_stemmers::{Algorithm, Stemmer};
use unicode_segmentation::UnicodeSegmentation;

pub fn process_text(text: &str) -> String {
    // Get the language of the text
    // let language = whatlang::detect(text).unwrap().lang();

    // Step 1: Convert text to lowercase
    let lowercased_text = text.to_lowercase();

    // Step 2: Remove any unwanted characters, such as punctuation or special characters
    let cleaned_text = remove_unwanted_characters(&lowercased_text);

    // Step 3: Tokenize the text into words (e.g., using an NLP library or custom function)
    let tokens = tokenize(&cleaned_text);

    // Step 4: Remove stop words
    let tokens_without_stop_words = remove_stop_words(tokens);

    // Step 4: Stemming
    // Stemming is the process of reducing a word to its word stem
    // let stemmed_tokens = stem_tokens(tokens, language);

    // Step 5: Join the tokens back into a single string
    tokens_without_stop_words.join(" ")
}

fn remove_stop_words(tokens: Vec<String>) -> Vec<String> {
    let stops = stop_words::get(stop_words::LANGUAGE::English);
    let mut tokens_without_stop_words: Vec<String> = Vec::new();
    for token in tokens {
        if !stops.contains(&token) {
            tokens_without_stop_words.push(token);
        }
    }
    tokens_without_stop_words
}

fn stem_tokens(tokens: Vec<String>, language: whatlang::Lang) -> Vec<String> {
    let lang_stemmer = match language {
        whatlang::Lang::Eng => Stemmer::create(Algorithm::English),
        whatlang::Lang::Spa => Stemmer::create(Algorithm::Spanish),
        _ => Stemmer::create(Algorithm::English),
    };

    let mut stemmed_tokens = Vec::new();
    for token in tokens {
        let stemmed_token = lang_stemmer.stem(&token).into_owned();
        stemmed_tokens.push(stemmed_token);
    }
    stemmed_tokens
}

/// Tokenizes the text into words
///
/// # Arguments
/// * `cleaned_text` - The text to tokenize
///
/// # Returns
/// * A vector of tokens
fn tokenize(cleaned_text: &str) -> Vec<String> {
    // Split the text into tokens using unicode word boundaries
    // e.g., "Hello, world!" -> ["Hello", ",", "world", "!"]
    // We want to keep punctuation so that we can use it for phrase queries
    let tokens = cleaned_text.split_word_bounds();
    let mut vector_tokens: Vec<String> = tokens.map(|token| token.to_owned()).collect();
    vector_tokens.retain(|token| !token.contains(" "));
    vector_tokens
}

/// Removes any characters that are not alphanumeric from the text
/// it keeps spaces and punctuation
///
/// # Arguments
///  * `lowercased_text` - The text to remove unwanted characters from
///
/// # Returns
/// * A new string with the unwanted characters removed from the text
fn remove_unwanted_characters(lowercased_text: &str) -> String {
    let mut cleaned_text = String::new();

    for character in lowercased_text.chars() {
        if character.is_alphanumeric() || character.is_whitespace() {
            cleaned_text.push(character);
        }
    }

    // Trim leading and trailing whitespace
    cleaned_text = cleaned_text.trim().to_owned();

    cleaned_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_unwanted_characters() {
        let tests: Vec<(&str, &str)> = vec![
            ("Hello, world!", "Hello world"),
            ("Hello, world! 123", "Hello world 123"),
            ("Hello, world! 123 $%^&*()", "Hello world 123"),
        ];

        for (input, expected_output) in tests {
            let output = remove_unwanted_characters(input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_tokenize() {
        let tests: Vec<(&str, Vec<&str>)> = vec![
            ("Hello, world!", vec!["Hello", ",", "world", "!"]),
            ("Hello, world! 123", vec!["Hello", ",", "world", "!", "123"]),
            (
                "Hello, world! 123 $%^&*()",
                vec![
                    "Hello", ",", "world", "!", "123", "$", "%", "^", "&", "*", "(", ")",
                ],
            ),
        ];

        for (input, expected_output) in tests {
            let output = tokenize(input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_process_text() {
        let input = "A blue car!";
        let expected_output = "blue car";
        let output = process_text(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn process_text_in_file() {
        let file_path = "data/blue_car.txt";
        let contents = std::fs::read_to_string(file_path).unwrap();
        let processed_text = process_text(&contents);
        assert_eq!(processed_text, "blue car");
    }

    #[test]
    fn test_process_spanish_text() {
        let input = "Hola, mundo este idioma es español!";
        let expected_output = "hola mundo este idioma es español";
        let output = process_text(input);
        assert_eq!(output, expected_output);
    }
}
