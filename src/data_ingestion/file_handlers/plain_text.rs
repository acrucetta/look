use crate::data_ingestion::file_handler::FileHandler;
use std::fs;

pub struct PlainTextHandler;

impl FileHandler for PlainTextHandler {
    fn can_handle(&self, file_extension: &str) -> bool {
        file_extension == "txt"
    }

    fn read_contents(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        Ok(content)
    }
}
