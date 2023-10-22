/// ## list_files(path: &str)
/// Lists all files in a given directory
///
/// # Arguments
///    path: &str - The path to the directory to list files in
///
/// # Returns
///   Vec<String> - A vector of strings containing the files
#[allow(dead_code)]
pub fn list_files(path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    let paths = std::fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            files.push(path.display().to_string());
        }
    }
    files
}

/// ## read_file(file: &str)
/// This function reads a file and returns a list of lines in the file
///
/// ## Arguments
/// - file: &str - The path to the file to read
///
/// ## Returns
/// - Vec<String> - A vector of strings containing the lines in the file
#[allow(dead_code)]
pub fn read_file(file: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}
