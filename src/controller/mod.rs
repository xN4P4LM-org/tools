use crate::helpers;
use crate::CONFIG;
mod down;
mod init;
mod new;
mod setup;
mod up;
mod watch;

/// ## run()
/// This function is the entry point for the controller module.
/// It takes a command and a list of arguments and runs the
/// appropriate function.
///
/// ### Arguments
/// - command: String - The command to run
/// - args: &[String] - A list of arguments to pass to the command
///
/// ### Returns
/// This function does not return anything
pub fn run(command: String, args: &[String]) {
    match command.as_str() {
        "down" => down(args),
        "init" => init(args),
        "up" => up(args),
        "watch" => watch(args),
        "status" => status(args),
        "-v" | "--version" => {
            // print the version number of the program
            println!("Tools Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Project Version: {}", CONFIG.get_version());
        }
        _ => {
            helpers::help::main_help::print_help();
        }
    }
}

// Controller actions

/// ## down()
/// This function stops the project using `docker compose down`
/// to stop the project.
///
/// ### Arguments
/// - args: &[String] - A list of arguments to pass to the command
///
/// ### Returns
/// This function does not return anything
pub fn down(args: &[String]) {
    println!("Not implemented up yet");
    match args.get(0) {
        Some(arg0) => match arg0.as_str() {
            // print the help message for the new command
            "help" | "-h" | "--help" => helpers::help::help_not_implemented_yet(),
            // if the argument is not recognized, print the help message for the new command
            _ => helpers::help::help_not_implemented_yet(),
        },
        None => {
            // if no arguments are passed, print the help message for the new command
            helpers::help::help_not_implemented_yet();
        }
    }
}

/// ## init()
/// This function initializes the project and creates the necessary
/// files and directories for the project. It also bootstraps the
/// project configs and secrets from the template files.
///
/// ### Arguments
/// - args: &[String] - A list of arguments to pass to the command
///
/// ### Returns
/// This function does not return anything
pub fn init(args: &[String]) {
    match args.get(0) {
        Some(arg0) => match arg0.as_str() {
            // rebuild_full performs a clean and then a init to rebuild the project
            // from scratch, it takes an argument to confirm the rebuild
            "rebuild" => init::rebuild_full(&args[1..]),
            // clean takes arguments to confirm the deletion
            "clean" => init::clean(&args[1..]),
            // print the help message for the init command
            "help" | "-h" | "--help" => helpers::help::init_help::print_help(),
            // if the argument is not recognized, print the help message for the init command
            _ => helpers::help::init_help::print_help(),
        },
        None => {
            // if no arguments are passed, run the init command with no arguments
            init::init();
        }
    }
}

// ## new()
// This function creates a new submodule for the project
//
// ### Arguments
// - args: &[String] - A list of arguments to pass to the command
//
// ### Returns
// This function does not return anything
pub fn new(args: &[String]) {
    println!("Not implemented up yet");
    match args.get(0) {
        Some(arg0) => match arg0.as_str() {
            // print the help message for the new command
            "help" | "-h" | "--help" => helpers::help::help_not_implemented_yet(),
            // if the argument is not recognized, print the help message for the new command
            _ => helpers::help::help_not_implemented_yet(),
        },
        None => {
            // if no arguments are passed, print the help message for the new command
            helpers::help::help_not_implemented_yet();
        }
    }
}

/// ## up()
/// This function starts the project using `docker compose up -d`
/// to run the project in the background.
///
/// ### Arguments
/// - args: &[String] - A list of arguments to pass to the command
///
/// ### Returns
/// This function does not return anything
pub fn up(args: &[String]) {
    println!("Not implemented up yet");
    match args.get(0) {
        Some(arg0) => match arg0.as_str() {
            // print the help message for the new command
            "help" | "-h" | "--help" => helpers::help::help_not_implemented_yet(),
            // if the argument is not recognized, print the help message for the new command
            _ => helpers::help::help_not_implemented_yet(),
        },
        None => {
            // if no arguments are passed, print the help message for the new command
            helpers::help::help_not_implemented_yet();
        }
    }
}

/// ## watch()
/// This function brings up the project and watches for changes in
/// the application code and restarts the project when changes are
/// detected using `docker compose up -d` and `docker compose down`.
///
/// ### Arguments
/// - args: &[String] - A list of arguments to pass to the command
///
/// ### Returns
/// This function does not return anything
pub fn watch(args: &[String]) {
    println!("Not implemented up yet");
    match args.get(0) {
        Some(arg0) => match arg0.as_str() {
            // print the help message for the new command
            "help" | "-h" | "--help" => helpers::help::help_not_implemented_yet(),
            // if the argument is not recognized, print the help message for the new command
            _ => helpers::help::help_not_implemented_yet(),
        },
        None => {
            // if no arguments are passed, print the help message for the new command
            helpers::help::help_not_implemented_yet();
        }
    }
}

// ## status()
// This function prints the status of the project
//
// ### Arguments
// - args: &[String] - A list of arguments to pass to the command
//
// ### Returns
// This function does not return anything
pub fn status(args: &[String]) {
    println!("Not implemented up yet");
    match args.get(0) {
        Some(arg0) => match arg0.as_str() {
            // print the help message for the new command
            "help" | "-h" | "--help" => helpers::help::help_not_implemented_yet(),
            // if the argument is not recognized, print the help message for the new command
            _ => helpers::help::help_not_implemented_yet(),
        },
        None => {
            // if no arguments are passed, print the help message for the new command
            helpers::help::help_not_implemented_yet();
        }
    }
}
