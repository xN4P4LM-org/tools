use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

/// This struct represents the structure of the project and
/// is used to serialize and deserialize the project definition file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub domain: String,
    pub version: String,
    pub services: HashMap<String, Service>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
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
