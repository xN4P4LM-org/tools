/// ## get_path(path: &str)
/// This function gets the path of a file or directory
///
/// ## Arguments
/// - path: &str - The path to the file or directory
///
/// ## Returns
/// - PathBuf - The path to the file or directory
#[allow(dead_code)]
pub fn get_path(path: &str) -> std::path::PathBuf {
    let path = std::fs::canonicalize(path).unwrap();
    path
}

/// ## append_path(base_path: &PathBuf, file: &str)
/// This function appends a file or directory to a path
///
/// ## Arguments
/// - base_path: &PathBuf - The base path to append the file or directory to
/// - file: &str - The file or directory to append to the base path
///
/// ## Returns
/// - PathBuf - The path to the file or directory
pub fn append_path(base_path: &std::path::PathBuf, file: &str) -> std::path::PathBuf {
    let file_path = base_path.join(file);
    file_path
}
