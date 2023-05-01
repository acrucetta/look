use unicode_segmentation::UnicodeSegmentation;

pub fn process_text(text: &str) -> String {
    // Step 1: Convert text to lowercase
    let lowercased_text = text.to_lowercase();

    // Step 2: Remove any unwanted characters, such as punctuation or special characters
    let cleaned_text = remove_unwanted_characters(&lowercased_text);

    // Step 3: Tokenize the text into words (e.g., using an NLP library or custom function)
    let tokens = tokenize(&cleaned_text);

    // Step 4: Stemming
    // TODO: Removed stemming for now, might add it back later

    tokens.join("")
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

    // Return the tokens as a vector
    tokens.map(|token| token.to_owned()).collect()
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
            let output = tokenize(input);
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
    fn process_text_in_file() {
        let file_path = "data/lorem_ipsum.txt";
        let contents = std::fs::read_to_string(file_path).unwrap();
        let processed_text = process_text(&contents);
        assert_eq!(processed_text, "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum");
    }

    #[test]
    fn test_process_spanish_text() {
        let input = "Hola, mundo este idioma es español!";
        let expected_output = "hola mundo este idioma es español";
        let output = process_text(input);
        assert_eq!(output, expected_output);
    }
}
