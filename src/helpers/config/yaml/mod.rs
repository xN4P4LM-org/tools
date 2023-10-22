use crate::models::app_config::{AppConfigFile, Version};
use std::{fs::File, io::Write, path::Path};

/// Retrieves the application configuration, either by reading it from a YAML file
/// or by creating a default configuration if the file does not exist.
///
/// ## Returns
/// - `AppConfigFile`: The application configuration.
///
/// ## Panics
/// - If the configuration file cannot be read.
/// - If the configuration file content cannot be deserialized.
/// - If the configuration file cannot be created.
/// - If the configuration cannot be serialized to YAML.
/// - If the YAML data cannot be written to the file.
pub fn get_config() -> AppConfigFile {
    let config_file = "config.yaml";

    // get current directory path
    let current_dir = std::env::current_dir().expect("Couldn't get current directory");

    // get the path to the configuration file
    let config_path = crate::helpers::filesystem::path::append_path(&current_dir, config_file);

    // Check if the configuration file already exists
    if !config_path.exists() {
        // If the file does not exist, create a default configuration
        let version = Version::init(0, 1, 0);
        let config = AppConfigFile::init(
            current_dir.to_str().unwrap().to_string(),
            "docker-compose.yaml".to_string(),
            ".gitmodules".to_string(),
            "project".to_string(),
            version,
        );

        // Write the default configuration to a YAML file
        write_config(&config);

        // Return the default configuration
        config
    } else {
        // If the file exists, read the configuration from the file
        let read_config = std::fs::read_to_string(config_path).expect("Couldn't read config file");

        // Deserialize the YAML data to an `AppConfigFile` instance
        let mut config_file: AppConfigFile =
            serde_yaml::from_str(&read_config).expect("Couldn't deserialize config file");

        if config_file.project_path == "." {
            // If the project path in the configuration file is a . (dot), update it
            // to the current directory in the configuration file
            config_file.project_path = current_dir.to_str().unwrap().to_string();
        }

        // Return the configuration
        config_file
    }
}

/// Writes the application configuration to a YAML file.
///
/// ## Arguments
/// - `config: &AppConfigFile`: A reference to the application configuration.
///
/// ## Panics
/// - If the file cannot be created.
/// - If the configuration cannot be serialized to YAML.
/// - If the YAML data cannot be written to the file.
pub fn write_config(config: &AppConfigFile) {
    // Serialize the configuration to a YAML string
    let config_yaml = serde_yaml::to_string(config).expect("Couldn't serialize config to YAML");

    // Create the YAML file
    let mut file = File::create(Path::new("config.yaml")).expect("Couldn't create file");

    // Write the YAML string to the file
    file.write_all(config_yaml.as_bytes())
        .expect("Couldn't write config to file");
}
