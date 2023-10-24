use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

/// This struct represents the structure of the project and
/// is used to serialize and deserialize the project definition file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String, // required - This is the name of the project and will be used to name the repository
    pub domain: String, // required - This is the domain of the project and can optionally be appended to the name of the project
    pub version: String, // required - This is the version of the project
    pub description: String, // required - This is the description of the project that serves as the description of the repository
    pub languages: Option<Vec<String>>, // optional - This is a list of programming languages used in the service
    pub frameworks: Option<Vec<String>>, // optional - This is the framework the service is developed
    pub services: Option<Vec<Project>>, // required for parent projects, or hierarchical projects - This is the list of services/children in the project
    pub repo: Option<Vec<ProjectRepository>>, // optional - This is the configuration for the repository
    pub parent: Option<Box<Project>>, // required for hierarchical projects - This is the parent project of the microservice
    pub from_template: Option<bool>, // optional - This indicates if the repository was created from a templates
    pub template: Option<String>, // optional - This is the name of the source template repository
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectRepository {
    pub name: String,                // required - This is the name of the repository
    pub description: String,         // required - This is the description of the repository
    pub provider: Option<String>, // optional - This is the name of the provider of the repository
    pub is_private: Option<bool>, // optional - This indicates if the repository is private
    pub web_url: Option<String>,  // optional - This is the web URL of the repository
    pub git_url: Option<String>,  // optional - This is the git URL of the repository
    pub from_template: Option<bool>, // optional - This indicates if the repository was created from a template
    pub is_template: Option<String>, // optional - This is the name of the source template repository
}

impl Project {
    pub fn from_yaml(yaml: &str) -> Result<Self, Box<dyn Error>> {
        let project: Project = serde_yaml::from_str(yaml)?;
        Ok(project)
    }

    pub fn to_yaml(&self) -> Result<String, Box<dyn Error>> {
        let mut yaml = serde_yaml::to_string(self)?;
        yaml.insert_str(0, "---\n");
        Ok(yaml)
    }

    /// This function updates the project version.
    pub fn update_version(&mut self, version: &Version) {
        self.version = version.to_string();
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
        Version::parse(&self.version).unwrap()
    }
}

#[test]
fn test_project() {
    let project = r#"---
name: test-project
domain: test-domain.test
version: 0.1.0
description: This is a test project
languages:
    - rust
    - javascript
frameworks:
    - actix
    - react
    - nextjs
services:
    - name: test-backend
      domain: test-domain.test
      version: 0.1.0
      description: This is a test backend service
      languages:
          - rust
      frameworks:
          - actix
      repo:
          - name: test-service
            description: This is a test service
            provider: github
            is_private: false
            web_url: https://github.com/github
    - name: test-frontend
      domain: test-domain.test
      version: 0.1.0
      description: This is a test frontend service
      languages:
          - javascript
      frameworks:
          - react
          - nextjs
      repo:
          - name: test-service
            description: This is a test service
            provider: github
            is_private: false
            web_url: https://github.com/github
"#;

    let project_from_yaml = Project::from_yaml(project);

    match project_from_yaml {
        Ok(_project) => {
            assert!(true)
        }
        Err(error) => {
            assert!(false, "Error: {}", error)
        }
    }
}

#[test]
fn test_project_read_from_file() {
    let project_from_file = fs::read_to_string("src/test/models/project/project.yaml").unwrap();

    let from_file = Project::from_yaml(&project_from_file);

    match from_file {
        Ok(_project) => assert!(true),
        Err(e) => assert!(false, "{}", e.to_string()),
    }
}
