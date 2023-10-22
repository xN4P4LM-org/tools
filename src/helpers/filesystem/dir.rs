use crate::CONFIG;

/// ## get_directories(path: Option<&str>)
/// Recursively gets all directories in the root path and defaults to . if no path is provided
///
/// ### Arguments
/// - path: Option<&str> - The path to get directories from (defaults to .)
///
/// ### Returns
/// - Vec<String> - A vector of strings containing the directories
#[allow(dead_code)]
pub fn get_directories(path: Option<&str>) -> Vec<String> {
    let mut directories: Vec<String> = Vec::new();
    let path = match path {
        Some(path) => path,
        None => ".",
    };
    let paths = std::fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            directories.push(path.display().to_string());
            directories.append(&mut get_directories(Some(
                path.display().to_string().as_str(),
            )));
        }
    }
    directories
}

/// ## check_if_directory_exists(path: &str)
/// This function checks if a directory exists
///
/// ### Arguments
/// - path: &str - The path to the directory
///
/// ### Returns
/// - bool - Whether or not the directory exists
pub fn check_if_directory_exists(path: &str) -> bool {
    // check if path is from root, if not, prepend the project path
    let path = if path.starts_with("/") {
        path.to_string()
    } else {
        format!("{}/{}", CONFIG.project_path, path)
    };

    let path = std::path::Path::new(&path);
    path.exists()
}

/// ## create_directory(path: &str)
/// This function creates a directory
///
/// ### Arguments
/// - path: &str - The path to the directory
///
/// ### Returns
/// - bool - Whether or not the directory was created
#[allow(dead_code)]
pub fn create_directory(path: &str) -> bool {
    let path = std::path::Path::new(path);
    match std::fs::create_dir(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// # (THIS FUNCTION IS DANGEROUS AND SHOULD BE USED WITH *EXTREME* CAUTION)
/// ## recursively_delete_directory(path: &str)
/// This function recursively deletes a directory
///
/// ### Arguments
/// - path: &str - The path to the directory
///
/// ### Returns
/// - bool - Whether or not the directory was deleted
#[allow(dead_code)]
pub fn recursively_delete_directory(path: &str) -> bool {
    let path = std::path::Path::new(path);
    match std::fs::remove_dir_all(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// ## get_absolute_path(path: &str)
/// This function gets the absolute path of a directory
///
/// ### Arguments
/// - path: &str - The path to the directory
///
/// ### Returns
/// - String - The absolute path of the directory
pub fn get_absolute_path(path: &str) -> String {
    // check if the path starts with root, and if not, prepend the project path
    let path = if path.starts_with("/") {
        path.to_string()
    } else {
        format!("{}/{}", CONFIG.project_path, path)
    };

    // return the absolute path without checking if it exists
    path
}

/// ## map_dirs(list_dirs: Vec<String>) -> Vec<(String, String, bool)>
/// This function maps a list of directories to a tuple of the absolute path, the path, and if the directory exists
///
/// ### Arguments
/// - list_dirs: Vec<String> - A list of directories
///
/// ### Returns
/// - Vec<(String, String, bool)> - A vector of tuples containing the absolute path, the path, and if the directory exists
pub fn map_dirs(list_dirs: Vec<String>) -> Vec<(String, String, bool)> {
    let maped_dirs = list_dirs
        .iter()
        .map(|path| {
            // return the tuple
            (
                get_absolute_path(path),
                path.clone(),
                check_if_directory_exists(path.as_str()),
            )
        })
        .collect::<Vec<(String, String, bool)>>();

    maped_dirs
}
