// Module: controller::init
mod submodules;

use submodules::{check_if_submodule_init_run_needed, get_submodule_paths};

/// ## init_new()
/// This version of the `init_new()` creates any missing directories
/// and files. This is useful when the project is already initialized
/// but the user wants to add a new service to the project.
///
/// returns: ()
pub fn init_new() {
    // check if all submodules have been initialized
    check_if_submodule_init_run_needed(get_submodule_paths());
}

/// ## rebuilt()
/// This function deletes all the items created by `init()` and then
/// reinitializes the project and creates the necessary files and
/// directories for the project.
///
/// returns: ()
pub fn rebuild_full() {
    println!("Not implemented rebuild full yet");
}

/// ## init()
/// This function initializes the project and creates the necessary
/// files and directories for the project.
///
/// returns: ()
pub fn init() {
    // check if all submodules have been initialized
    check_if_submodule_init_run_needed(get_submodule_paths());
}
