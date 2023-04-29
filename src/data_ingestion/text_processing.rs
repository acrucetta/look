use rust_stemmers::{Algorithm, Stemmer};
use tokenizers::tokenizer::Tokenizer;

pub fn process_text(text: &str) -> String {
    // Step 1: Convert text to lowercase
    let lowercased_text = text.to_lowercase();

    // Step 2: Remove any unwanted characters, such as punctuation or special characters
    let cleaned_text = remove_unwanted_characters(&lowercased_text);

    // Step 3: Tokenize the text into words (e.g., using an NLP library or custom function)
    let tokens = tokenize(cleaned_text);

    // Step 4: Apply stemming or lemmatization to the tokens (e.g., using an NLP library or stemming/lemmatization library)
    let processed_tokens = apply_stemming_or_lemmatization(tokens);

    // Step 5: Join the processed tokens back into a single string

    processed_tokens.join(" ")
}

/// Applies stemming or lemmatization to the tokens
///
/// # Arguments
///  * `tokens` - The tokens to apply stemming or lemmatization to
///
/// # Returns
///
/// * A vector of tokens with stemming or lemmatization applied
///
fn apply_stemming_or_lemmatization(tokens: Vec<String>) -> Vec<String> {
    // Step 1: Create a new stemmer
    let stemmer = Stemmer::create(Algorithm::English);

    // Step 2: Apply stemming to each token
    let processed_tokens: Vec<String> = tokens
        .iter()
        .map(|token| stemmer.stem(token).to_string())
        .collect();

    // Step 3: Return the processed tokens
    processed_tokens
}

/// Tokenizes the text into words
///
/// # Arguments
/// * `cleaned_text` - The text to tokenize
///
/// # Returns
/// * A vector of tokens
fn tokenize(cleaned_text: String) -> Vec<String> {
    // Step 1: Create a new tokenizer, using the default settings
    let tokenizer = Tokenizer::from_pretrained("bert-base-uncased", None).unwrap();

    // Step 2: Encode the cleaned text}
    let encoding = tokenizer.encode(cleaned_text, false).unwrap();

    // Step 3: Return the tokens
    encoding.get_tokens().to_vec()
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

    cleaned_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_unwanted_characters() {
        let tests: Vec<(&str, &str)> = vec![
            ("Hello, world!", "Helloworld"),
            ("Hello, world! 123", "Helloworld123"),
            ("Hello, world! 123 $%^&*()", "Helloworld123"),
        ];

        for (input, expected_output) in tests {
            let output = remove_unwanted_characters(input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_tokenize() {
        let tests: Vec<(&str, Vec<&str>)> = vec![
            ("Hello, world!", vec!["hello", ",", "world", "!"]),
            ("Hello, world! 123", vec!["hello", ",", "world", "!", "123"]),
            (
                "Hello, world! 123 $%^&*()",
                vec![
                    "hello", ",", "world", "!", "123", "$", "%", "^", "&", "*", "(", ")",
                ],
            ),
        ];

        for (input, expected_output) in tests {
            let output = tokenize(input.to_string());
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_process_text() {
        let input = "Hello, world!";
        let expected_output = "hello world";
        let output = process_text(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_stemmer() {
        let stemmer = Stemmer::create(Algorithm::English);
        let tests: Vec<(&str, &str)> = vec![
            ("fruitless", "fruitless"),
            ("fruitlessly", "fruitless"),
            ("fruitlessness", "fruitless"),
            ("fruition", "fruition"),
        ];

        for (input, expected_output) in tests {
            let output = stemmer.stem(input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn process_text_in_file() {
        let file_path = "data/lorem_ipsum.txt";
        let contents = std::fs::read_to_string(file_path).unwrap();
        let processed_text = process_text(&contents);
        assert_eq!(processed_text, "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum");
    }
}
