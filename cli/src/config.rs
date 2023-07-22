use std::{io::Write, path::Path};

pub struct Config {
    pub index_path: String,
    pub personal_data: String,
    pub app_config_path: String,
}

pub fn load_config() -> Config {
    // We're going to use the directories crate to find the config dir
    let config_path = directories::BaseDirs::new()
        .unwrap()
        .config_dir()
        .to_str()
        .unwrap()
        .to_owned()
        + "/looker-cli";

    // If we don't have an app folder, create one
    if !Path::new(&config_path).exists() {
        std::fs::create_dir(&config_path).unwrap();
    }

    // Load the config file with Config::builder
    let settings_path = config_path.clone() + "/.env";
    let index_path = config_path.clone() + "/index.json";
    let personal_data = config_path.clone() + "/personal_data";

    // Create a new config file if it doesn't exist
    if !Path::new(&settings_path).exists() {
        let mut settings_file = std::fs::File::create(&settings_path).unwrap();
        // Write the default settings to the file
        settings_file
            .write_all(
                format!(
                    "INDEX_PATH='{}'\nPERSONAL_DATA='{}'",
                    index_path, personal_data
                )
                .as_bytes(),
            )
            .unwrap();
    }
    // Load the config file
    dotenv::from_path(&settings_path).unwrap();

    Config {
        index_path: std::env::var("INDEX_PATH").unwrap(),
        personal_data: std::env::var("PERSONAL_DATA").unwrap(),
        app_config_path: config_path,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        load_config();
        let index_path = std::env::var("INDEX_PATH").unwrap();
        let personal_data = std::env::var("PERSONAL_DATA").unwrap();

        assert_eq!(
            index_path,
            "Users/andrescrucettanieto/Library/Application Support/looker-cli/index.json"
        );
        assert_eq!(
            personal_data,
            "Users/andrescrucettanieto/Library/Application Support/looker-cli/personal_data"
        );
    }
}
