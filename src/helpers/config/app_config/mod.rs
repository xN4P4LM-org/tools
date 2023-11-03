use semver::{Prerelease, Version};

use crate::helpers::filesystem::path::append_path;
use crate::models::app_config::AppConfigFile;
use std::env::current_dir;
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
    let current_dir = current_dir().expect("Couldn't get current directory");

    // get the path to the configuration file
    let config_path = append_path(&current_dir, config_file);

    // Check if the configuration file already exists
    if !config_path.exists() {
        // If the file does not exist, create a default configuration
        let config = AppConfigFile::default();

        // Write the configuration to a YAML file
        write_config(&config);

        // Return the configuration
        config
    } else {
        // If the file exists, read the configuration from the file
        let raw_config_file =
            std::fs::read_to_string(config_path).expect("Couldn't read config file");

        // Deserialize the configuration from YAML
        let config_file = AppConfigFile::from_yaml(&raw_config_file);

        match config_file {
            Ok(config) => config,
            Err(e) => {
                println!("Couldn't deserialize config file: {}", e);
                std::process::exit(1);
            }
        }
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
    let config_yaml = config.to_yaml().expect("Couldn't serialize config to YAML");

    // Create the YAML file
    let mut file = File::create(Path::new("config.yaml")).expect("Couldn't create file");

    // Write the YAML string to the file
    file.write_all(config_yaml.as_bytes())
        .expect("Couldn't write config to file");
}

/// Create a default configuration file.
///
/// ## Arguments
/// - `config_file: &str`: The path to the configuration file.
///
/// ## Returns
/// - `AppConfigFile`: The default configuration.
pub fn create_config(config_file: &str) -> AppConfigFile {
    // get current directory path
    let current_dir = current_dir().expect("Couldn't get current directory");

    // get the path to the configuration file
    let config_path = append_path(&current_dir, config_file);

    // If the file does not exist, create a default configuration
    let version = Version {
        major: 0,
        minor: 1,
        patch: 0,
        pre: Prerelease::new("alpha").unwrap(),
        build: semver::BuildMetadata::EMPTY,
    }
    .to_string();
    let config = AppConfigFile {
        project_path: current_dir.to_str().unwrap().to_string(),
        docker_compose: "docker-compose.yaml".to_string(),
        project_name: "project".to_string(),
        project_version: version,
        github_api_token: None,
    };

    // Write the default configuration to a YAML file
    write_config(&config);

    // Return the default configuration
    config
}

/// Check if the configuration file exists.
///
/// ## Returns
/// - `bool`: `true` if the configuration file exists, `false` otherwise.
pub fn check_if_config_exists() -> bool {
    // get current directory path
    let current_dir = current_dir().expect("Couldn't get current directory");

    // get the path to the configuration file
    let config_path = append_path(&current_dir, "config.yaml");

    // Check if the configuration file already exists
    if !config_path.exists() {
        false
    } else {
        true
    }
}
