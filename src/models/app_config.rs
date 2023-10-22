use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfigFile {
    pub project_path: String,
    pub docker_compose: String,
    pub gitmodules: String,
    pub project_name: String,
    pub project_version: Version,
}

impl AppConfigFile {
    pub fn init(
        project_path: String,
        docker_compose: String,
        gitmodules: String,
        project_name: String,
        project_version: Version,
    ) -> AppConfigFile {
        AppConfigFile {
            project_path,
            docker_compose,
            gitmodules,
            project_name,
            project_version,
        }
    }

    #[allow(dead_code)]
    pub fn get_project_version(&self) -> String {
        self.project_version.get_project_version()
    }

    #[allow(dead_code)]
    pub fn update_project_version(&mut self, version: Version) {
        self.project_version = version;
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    pub fn init(major: u8, minor: u8, patch: u8) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn get_project_version(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
