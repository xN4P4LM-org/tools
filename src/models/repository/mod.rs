use std::{error::Error, fs};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Repository {
    id: i32,
    node_id: String,
    name: String,
    full_name: String,
    owner: Owner,
    private: Option<bool>,
    html_url: Option<String>,
    description: Option<String>,
    fork: Option<bool>,
    url: Option<String>,
    language: Option<String>,
    forks_count: Option<i32>,
    stargazers_count: Option<i32>,
    watchers_count: Option<i32>,
    size: Option<i32>,
    default_branch: Option<String>,
    open_issues_count: Option<i32>,
    is_template: Option<bool>,
    topics: Option<Vec<String>>,
    has_issues: Option<bool>,
    has_projects: Option<bool>,
    has_wiki: Option<bool>,
    has_pages: Option<bool>,
    has_downloads: Option<bool>,
    has_discussions: Option<bool>,
    archived: Option<bool>,
    disabled: Option<bool>,
    visibility: Option<String>,
    pushed_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Owner {
    login: String,
    id: i32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    #[serde(rename = "type")]
    owner_type: String,
    site_admin: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct License {
    key: String,
    name: String,
    url: String,
    spdx_id: String,
    node_id: String,
    html_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Permissions {
    admin: bool,
    push: bool,
    pull: bool,
}
/// This implementation block contains functions for the Repository struct for a GitHub repository.
/// It is used to parse the response from the JSON API.
impl Repository {
    /// This function returns the repository from a json formated text string.
    pub fn from_json(json: &str) -> Result<Self, Box<dyn Error>> {
        let repository: Repository = serde_json::from_str(json)?;
        Ok(repository)
    }

    /// This function returns the repository as a json formated text string.
    pub fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }
}

#[test]
fn test_creating_a_repository_from_json_string() {
    let json_structure: &str = r#"
{
    "id": 1024,
    "node_id": "i-am-a-node-id",
    "name": "test-repository",
    "full_name": "test-user/test-repository",
    "owner": {
        "login": "test-user",
        "id": 1210,
        "node_id": "i-am-a-node-id",
        "avatar_url": "String",
        "gravatar_id": "String",
        "type": "User",
        "site_admin": false
    }
}
"#;

    let repo_from_json = Repository::from_json(json_structure);

    match repo_from_json {
        Ok(_repository) => {
            assert!(true)
        }
        Err(error) => {
            assert!(false, "Error: {}", error)
        }
    }
}

#[test]
fn test_creating_a_repository_from_json_string_with_invalid_json() {
    let json_structure: &str = r#"
{
    "id": 1024,
    "node_id": "i-am-a-node-id",
    "name": "test-repository",
    "full_name": "test-user/test-repository",
    "owner": {
        "login": "test-user",
        "id": 1210,
        "node_id": "i-am-a-node-id",
        "avatar_url": "String",
        "gravatar_id": "String",
        "type": "User",
        "site_admin": false
    }
"#;

    let repo_from_json = Repository::from_json(json_structure);

    match repo_from_json {
        Ok(_repository) => {
            assert!(false, "Should have failed to parse the json string.")
        }
        Err(_error) => {
            assert!(true)
        }
    }
}

#[test]
fn test_reading_a_repository_from_a_file() {
    let repo_from_file =
        fs::read_to_string("src/test/models/repository/example_api_call.json").unwrap();

    let repo_from_json = Repository::from_json(&repo_from_file);

    match repo_from_json {
        Ok(_repository) => {
            assert!(true)
        }
        Err(error) => {
            assert!(false, "Error: {}", error)
        }
    }
}
