use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfigFile {
    pub project_path: String,
    pub docker_compose: String,
    pub gitmodules: String,
    pub project_name: String,
    pub project_version: String,
    pub github_api_token: Option<String>,
}

impl AppConfigFile {
    /// This function returns the application configuration from yaml formated text string.
    pub fn from_yaml(yaml: &str) -> Result<Self, Box<dyn Error>> {
        let app_config_file: AppConfigFile = serde_yaml::from_str(yaml)?;
        Ok(app_config_file)
    }

    /// This function returns the application configuration as a yaml formated text string.
    pub fn to_yaml(&self) -> Result<String, Box<dyn Error>> {
        let mut yaml = serde_yaml::to_string(self)?;
        yaml.insert_str(0, "---\n");
        Ok(yaml)
    }

    /// This function updates the project version.
    pub fn update_version(&mut self, version: &Version) {
        self.project_version = version.to_string();
    }

    /// This function updates the major version of the project version.
    pub fn update_major_version(&mut self, major: u64) {
        let mut version = self.get_version();
        version.major = major;
        self.update_version(&version);
    }

    /// This function updates the minor version of the project version.
    pub fn update_minor_version(&mut self, minor: u64) {
        let mut version = self.get_version();
        version.minor = minor;
        self.update_version(&version);
    }

    /// This function updates the patch version of the project version.
    pub fn update_patch_version(&mut self, patch: u64) {
        let mut version = self.get_version();
        version.patch = patch;
        self.update_version(&version);
    }

    /// This function updates the prerelease version of the project version.
    pub fn update_prerelease_version(&mut self, prerelease: &str) {
        let mut version = self.get_version();
        version.pre = Prerelease::new(prerelease).unwrap();
        self.update_version(&version);
    }

    /// This function updates the build metadata of the project version.
    pub fn update_build_version(&mut self, build: &str) {
        let mut version = self.get_version();
        version.build = BuildMetadata::new(build).unwrap();
        self.update_version(&version);
    }

    /// This function returns the project version as a semver::Version
    /// struct.
    pub fn get_version(&self) -> Version {
        Version::parse(&self.project_version).unwrap()
    }
}

#[test]
fn test_app_config() {
    let config = AppConfigFile {
        project_path: ".".to_string(),
        docker_compose: "docker-compose.yaml".to_string(),
        gitmodules: ".gitmodules".to_string(),
        project_name: "project".to_string(),
        project_version: Version {
            major: 0,
            minor: 1,
            patch: 0,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        }
        .to_string(),
        github_api_token: None,
    };

    let config_yaml = r#"---
project_path: .
docker_compose: docker-compose.yaml
gitmodules: .gitmodules
project_name: project
project_version: 0.1.0
github_api_token: null
"#;

    assert_eq!(config, AppConfigFile::from_yaml(config_yaml).unwrap());
    assert_eq!(config_yaml, config.to_yaml().unwrap());
}
