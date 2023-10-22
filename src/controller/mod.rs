use crate::helpers;

mod down;
mod init;
mod up;
mod watch;

/// ## run()
/// This function is the entry point for the controller module.
/// It takes a command and a list of arguments and runs the
/// appropriate function.
///
/// ## Arguments
/// - command: String - The command to run
/// - args: &[String] - A list of arguments to pass to the command
///
/// ## Returns
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
            print!("{}\n", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            helpers::help::main_help::print_help();
        }
    }
}

/// ## init()
/// This function initializes the project and creates the necessary
/// files and directories for the project. It also bootstraps the
/// project configs and secrets from the template files.
///
/// ## Arguments
/// - args: &[String] - A list of arguments to pass to the command
///
/// ## Returns
/// This function does not return anything
pub fn init(args: &[String]) {
    match args.get(0) {
        Some(arg0) => match arg0.as_str() {
            "rebuild" => init::rebuild_full(),
            "new" => init::init_new(),
            "help" | "-h" | "--help" => helpers::help::init_help::print_help(),
            _ => init::init(),
        },
        None => {
            init::init();
        }
    }
}

/// ## up()
/// This function starts the project using `docker compose up -d`
/// to run the project in the background.
///
/// ## Arguments
/// - args: &[String] - A list of arguments to pass to the command
///
/// ## Returns
/// This function does not return anything
pub fn up(args: &[String]) {
    println!("Not implemented up yet");
    // print the arguments
    println!("{:?}", args);
}

/// ## down()
/// This function stops the project using `docker compose down`
/// to stop the project.
///
/// ## Arguments
/// - args: &[String] - A list of arguments to pass to the command
///
/// ## Returns
/// This function does not return anything
pub fn down(args: &[String]) {
    println!("Not implemented down yet");
    // print the arguments
    println!("{:?}", args);
}

/// ## watch()
/// This function brings up the project and watches for changes in
/// the application code and restarts the project when changes are
/// detected using `docker compose up -d` and `docker compose down`.
///
/// ## Arguments
/// - args: &[String] - A list of arguments to pass to the command
///
/// ## Returns
/// This function does not return anything
pub fn watch(args: &[String]) {
    println!("Not implemented yet");
    // print the arguments
    println!("{:?}", args);
}

// ## status()
// This function prints the status of the project
//
// ## Arguments
// - args: &[String] - A list of arguments to pass to the command
//
// ## Returns
// This function does not return anything
pub fn status(args: &[String]) {
    println!("Not implemented yet");
    // print the arguments
    println!("{:?}", args);
}
