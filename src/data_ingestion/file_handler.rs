pub trait FileHandler {
    fn can_handle(&self, file_extension: &str) -> bool;
    fn read_contents(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>>;
}

/// Traverse a directory and return a list of file paths.
///
/// # Arguments
/// * `directory_path` - The path to the directory to traverse
///
/// # Returns
/// * A list of file paths
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
pub fn traverse_directory(
    directory_path: &str,
    file_handlers: &Vec<Box<dyn FileHandler>>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file_paths = Vec::new();

    for entry in std::fs::read_dir(directory_path)? {
        let entry = entry?;
        let file_path = entry.path();
        let file_path = file_path.to_str().unwrap();

        if entry.file_type()?.is_dir() {
            let mut subdirectory_file_paths = traverse_directory(file_path, file_handlers)?;
            file_paths.append(&mut subdirectory_file_paths);
        } else {
            file_paths.push(file_path.to_string());
        }
    }

    Ok(file_paths)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_traverse_directory() {
        let directory_path = "data";
        let file_handlers = Vec::new();
        let file_paths = super::traverse_directory(directory_path, &file_handlers).unwrap();
        assert_eq!(file_paths.len(), 3);
    }
}
