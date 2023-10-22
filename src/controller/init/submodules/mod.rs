use crate::helpers::filesystem::dir::map_dirs;
use crate::CONFIG;

// read the .gitmodules file and get the paths of the submodules
// example .gitmodules file:
// [submodule "backend"]
// path = backend
// url = git@github.com:xN4P4LM-org/{project}
pub fn get_submodule_paths() -> Vec<(String, String, bool)> {
    // read the config file and get the .gitmodules file path and root path
    let base_path = CONFIG.project_path.clone();
    let submodule_path = CONFIG.gitmodules.clone();

    // read the .gitmodules file and get the paths of the submodules
    // example .gitmodules file:
    // [submodule "backend"]
    // path = backend
    // url = git@github.com:xN4P4LM-org/{project}
    let sub_module_paths = std::fs::read_to_string(format!("{}/{}", base_path, submodule_path))
        .expect("Couldn't read .gitmodules file")
        .lines()
        .filter(|line| line.contains("path = "))
        .map(|line| line.replace("path = ", "").trim().to_string())
        .collect::<Vec<String>>();

    // return the submodule paths
    return map_dirs(sub_module_paths);
}

pub fn check_if_submodule_init_run_needed(sub_module_paths: Vec<(String, String, bool)>) {
    for dir in sub_module_paths {
        println!("{} - {} - {}", &dir.0, &dir.1, &dir.2);

        // if the directory doesn't exist, call `git submodule update --init --recursive`
        if !dir.2 {
            // run the command in a child process in the directory of the project
            let output = std::process::Command::new("git")
                .arg("submodule")
                .arg("update")
                .arg("--init")
                .arg("--recursive")
                .current_dir(CONFIG.project_path.clone())
                .output()
                .expect("Failed to run git submodule update --init --recursive");

            // print the output
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
