use crate::helpers::filesystem::dir::map_dirs;
use crate::CONFIG;

// ## get_submodule_paths() -> Vec<(String, String, bool)>
// This function reads the .gitmodules file and returns the paths of the submodules
//
// ### arguments:
// - nothing
//
// ### returns:
// - Vec<(String, String, bool)> - vector of tuples containing information about the submodules
//   - (String, String, bool) - (submodule name, submodule path, submodule initialized)
pub fn get_submodule_paths() -> Vec<(String, String, bool)> {
    // read the config file and get the .gitmodules file path and root path
    let base_path = CONFIG.project_path.clone();
    // try to get the path to .gitmodule file in the current directory
    let submodule_path = std::fs::canonicalize(format!("{}/{}", base_path, ".gitmodules"))
        .expect("Couldn't get path to .gitmodules file")
        .to_str()
        .unwrap()
        .to_string();

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
    map_dirs(sub_module_paths)
}

// check_if_submodule_init_run_needed(sub_module_paths: Vec<(String, String, bool)>)
// This function checks if the submodules are initialized and if not,
// it runs the command `git submodule update --init --recursive` in the project directory
//
// ### arguments:
// - sub_module_paths: Vec<(String, String, bool)> - vector of tuples containing information about the submodules
//   - (String, String, bool) - (submodule name, submodule path, submodule initialized)
//
// ### returns:
// - nothing
pub fn check_if_submodule_init_run_needed(sub_module_paths: Vec<(String, String, bool)>) -> bool {
    let mut missing_submodule = false;

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
                .output();

            // print the output
            match output {
                Ok(output) => {
                    if output.status.success() {
                        missing_submodule = true
                    } else {
                        eprintln!("Command executed with failing error code");
                        missing_submodule = true
                    }
                }
                Err(_) => {
                    eprintln!("Failed to run git submodule update --init --recursive");
                    missing_submodule = true
                }
            }
        }
    }

    // return if a submodule was missing
    missing_submodule
}

// ## delete_all_submodules()
// This function deletes all submodules to allow a clean re-initialization
// of the submodules
//
// ### arguments:
// - nothing
//
// ### returns:
// - bool - true if the deletion was successful, false if not
pub fn delete_all_submodules() -> bool {
    // get the submodule paths
    let sub_module_paths = get_submodule_paths();

    let mut success = false;

    // delete all submodules
    for dir in sub_module_paths {
        // delete the directory
        let removed_dir =
            std::fs::remove_dir_all(format!("{}/{}", CONFIG.project_path.clone(), dir.1));

        match removed_dir {
            Ok(_) => {
                println!("Removed submodule {}", dir.0);
                success = true;
            }
            Err(_) => {
                eprintln!("Failed to remove submodule {}", dir.0);
                success = false;
            }
        }
    }

    // return the outcome of the deletion
    success
}
