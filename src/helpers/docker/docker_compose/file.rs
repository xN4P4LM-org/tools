use crate::helpers::filesystem::path::append_path;
use crate::models::docker_compose::DockerCompose;
use crate::CONFIG;
use std::fs::read_to_string;
use std::path::PathBuf;

/// ## get_path_to_docker_compose() -> PathBuf
/// This function returns a PathBuf to the docker compose file
///
/// ### Returns
/// - PathBuf
fn get_path_to_docker_compose() -> PathBuf {
    let raw_project_path = &CONFIG.project_path;
    let project_path = PathBuf::from(raw_project_path);
    let docker_compose_filename = &CONFIG.docker_compose;
    let docker_compose_path = append_path(&project_path, docker_compose_filename);
    docker_compose_path
}

/// ## get_docker_compose_file() -> DockerCompose
/// This function returns a DockerComposeFile struct from a yaml string
///
/// ### Returns
/// - DockerComposeFile
pub fn get_docker_compose_file() -> DockerCompose {
    // get absolute path to docker compose file
    let docker_compose_path = get_path_to_docker_compose();

    // read docker compose into DockerCompose struct
    let docker_compose = DockerCompose::from_yaml(
        &read_to_string(docker_compose_path)
            .expect("Could not read docker compose file into string"),
    );

    match docker_compose {
        Ok(docker_compose) => docker_compose,
        Err(error) => panic!("Could not parse docker compose file: {}", error),
    }
}

/// ## write_docker_compose_file(docker_compose: &DockerCompose) -> bool
/// This function writes a DockerCompose struct to the docker compose file
///
/// ### Arguments
/// - docker_compose: &DockerCompose - DockerCompose struct to write to file
///
/// ### Returns
/// - bool: true if successful, false if not
pub fn write_docker_compose_file(docker_compose: &DockerCompose) -> bool {
    let docker_compose_path = get_path_to_docker_compose();
    let docker_compose_yaml = docker_compose.to_yaml().unwrap();
    std::fs::write(docker_compose_path, docker_compose_yaml).unwrap();
    true
}
