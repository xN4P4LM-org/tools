// Module: controller::init
mod submodules;
use self::submodules::delete_all_submodules;
use crate::helpers::user_input;
use submodules::{check_if_submodule_init_run_needed, get_submodule_paths};

/// ## rebuilt()
/// This function deletes all the items created by `init()` and then
/// reinitializes the project and creates the necessary files and
/// directories for the project.
///
/// ### returns: ()
pub fn rebuild_full(args: &[String]) {
    if args.get(0).map(|s| s.as_str()) == Some("-y") {
        clean_logic();
    } else {
        println!("Cleaning the project... Are you sure you want to do this?");
        println!("This will delete all the files and directories created by the init command.");

        if user_input::get_user_input_with_options("Continue? [y/N]: ", &["y"]) {
            println!("Cleaning the project...");
            clean_logic();
        } else {
            println!("Aborting...");
        }
    }

    // init the project
    init();
}

/// ## clean_logic()
/// this function contains the logic for the clean command and
/// deletes all the items created by `init()` leaving the
/// project in a uninitialized state.
///
/// ### returns: ()
fn clean_logic() {
    println!("Cleaning the project...");

    // delete all submodules
    delete_all_submodules();
}

/// ## clean()
/// This function
///
/// ### returns: ()
pub fn clean(args: &[String]) {
    // check if the -y flag was passed to the command
    // if it was not, ask the user for confirmation
    if args.get(0).map(|s| s.as_str()) == Some("-y") {
        clean_logic();
    } else {
        println!("Cleaning the project... Are you sure you want to do this?");
        println!("This will delete all the files and directories created by the init command.");

        if user_input::get_user_input_with_options("Continue? [y/N]: ", &["y"]) {
            println!("Cleaning the project...");

            // delete all submodules
            delete_all_submodules();
        } else {
            println!("Aborting...");
        }
    }
}

/// ## init()
/// This function initializes the project and creates the necessary
/// files and directories for the project.
///
/// ### returns: ()
pub fn init() {
    // check if all submodules have been initialized
    check_if_submodule_init_run_needed(get_submodule_paths());
}
