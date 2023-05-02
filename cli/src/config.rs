use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub index_path: String,
    pub data_path: String,
}

impl AppConfig {
    pub fn new(config_dir: Option<&Path>) -> Self {
        let mut settings = config::Config::default();
        settings.set_default("index_path", ".").unwrap();
        settings.set_default("data_path", ".").unwrap();

        let config_dir = config_dir
            .map(Path::to_path_buf)
            .unwrap_or_else(|| dirs_next::config_dir().unwrap().join("looker"));

        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir).unwrap();
        }

        let config_file = config_dir.join("settings.toml");
        if config_file.exists() {
            settings.merge(config::File::from(config_file)).unwrap();
        }

        settings.merge(config::Environment::new()).unwrap();
        settings.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_app_config_defaults() {
        let config = AppConfig::new(None);
        assert_eq!(config.index_path, ".");
        assert_eq!(config.data_path, ".");
    }

    #[test]
    fn test_app_config_from_file() {
        let temp_dir = tempdir().unwrap();
        let config_file = temp_dir.path().join("settings.toml");
        let mut file = File::create(&config_file).unwrap();

        writeln!(
            file,
            "index_path = 'custom_index'\ndata_path = 'custom_data'"
        )
        .unwrap();

        let config = AppConfig::new(Some(temp_dir.path()));
        assert_eq!(config.index_path, "custom_index");
        assert_eq!(config.data_path, "custom_data");
    }

    #[test]
    fn test_app_config_directory_creation() {
        let temp_dir = tempdir().unwrap();
        let config_dir = temp_dir.path().join("looker");

        AppConfig::new(Some(temp_dir.path()));

        assert!(config_dir.exists());
    }
}
