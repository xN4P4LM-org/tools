use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Build {
    context: String,
    dockerfile: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    build: Option<Build>,
    image: Option<String>,
    ports: Option<Vec<String>>,
    networks: Option<Vec<String>>,
    configs: Option<Vec<String>>,
    secrets: Option<Vec<String>>,
    volumes: Option<Vec<String>>,
    depends_on: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    driver: Option<String>,
    driver_opts: Option<HashMap<String, String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Network {
    driver: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    external: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Secret {
    external: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerCompose {
    services: HashMap<String, Service>,
    volumes: HashMap<String, Volume>,
    networks: HashMap<String, Network>,
    configs: HashMap<String, Config>,
    secrets: HashMap<String, Secret>,
}

impl DockerCompose {
    pub fn from_yaml(yaml: &str) -> Result<Self, Box<dyn Error>> {
        let docker_compose: DockerCompose = serde_yaml::from_str(yaml)?;
        Ok(docker_compose)
    }

    pub fn to_yaml(&self) -> Result<String, Box<dyn Error>> {
        let mut yaml = serde_yaml::to_string(self)?;
        yaml.insert_str(0, "---\n");
        Ok(yaml)
    }
}

#[test]
fn test_docker_compose() {
    let docker_compose = r#"---
services:
    web:
        build:
        context: ./web
        dockerfile: Dockerfile
        image: web:latest
        ports:
        - 8080:8080
        networks:
        - web
        configs:
        - web_config
        secrets:
        - web_secret
        volumes:
        - web_volume
        depends_on:
        - db
    db:
        build:
        context: ./db
        dockerfile: Dockerfile
        image: db:latest
        ports:
        - 5432:5432
        networks:
        - db
        configs:
        - db_config
        secrets:
        - db_secret
        volumes:
        - db_volume
networks:
    web:
        driver: bridge
    db:
        driver: bridge
configs:
    web_config:
        external: true
    db_config:
        external: true
secrets:
    web_secret:
        external: true
    db_secret:
        external: true
volumes:
    web_volume:
        driver: local
    db_volume:
        driver: local
"#;

    //relative path from the project root
    let path_to_test_docker_compose = "src/tests/models/docker_compose/docker-compose.yaml";

    // get directory where this program is running
    let current_dir = std::env::current_dir().unwrap();

    // get absolute path to path_to_test_docker_compose
    let absolue_path = std::path::Path::new(&current_dir).join(path_to_test_docker_compose);

    println!("absolue_path: {:?}", absolue_path);

    let docker_compose = DockerCompose::from_yaml(docker_compose).unwrap();

    // get docker_compose from path_to_test_docker_compose
    let yaml = std::fs::read_to_string(absolue_path).unwrap();

    let yaml_from_file = DockerCompose::from_yaml(&yaml).unwrap();

    assert_eq!(docker_compose, yaml_from_file);
}
