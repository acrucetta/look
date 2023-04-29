use rust_stemmers::{Algorithm, Stemmer};
use tokenizers::tokenizer::{EncodeInput, Result, Tokenizer};

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
    let processed_text = processed_tokens.join(" ");

    processed_text
}

fn apply_stemming_or_lemmatization(tokens: Vec<String>) -> Vec<String> {
    todo!()
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
///
/// # Arguments
///  * `lowercased_text` - The text to remove unwanted characters from
///
/// # Returns
/// * A new string with the unwanted characters removed from the text
fn remove_unwanted_characters(lowercased_text: &str) -> String {
    let mut cleaned_text = String::new();

    for character in lowercased_text.chars() {
        if character.is_alphanumeric() {
            cleaned_text.push(character);
        }
    }

    cleaned_text
}

#[cfg(test)]
mod tests {
    use super::*;
}
