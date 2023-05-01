pub trait FileHandler {
    fn can_handle(&self, file_extension: &str) -> bool;
    fn read_contents(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>>;
}
