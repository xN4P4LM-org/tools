use crate::{helpers::config, models::app_config::AppConfigFile};
use once_cell::sync::Lazy;

/// This is the main file for the program
mod controller;
mod helpers;
mod models;

pub static CONFIG: Lazy<AppConfigFile> = Lazy::new(|| config::yaml::get_config());

/// ## main()
/// This function is the entry point for the program
fn main() {
    // if the config project path is not the current directory,
    // warn the user that this may cause unexpected behavior
    if CONFIG.project_path
        != std::env::current_dir()
            .expect("Couldn't get current directory")
            .to_str()
            .unwrap()
            .to_string()
    {
        println!("WARNING: The project path in the configuration file is not the current directory. This may cause unexpected behavior.");
    }

    // switch statement based on the first argument and pass the rest of the arguments to the controller
    match std::env::args().nth(1) {
        Some(command) => {
            let args: Vec<String> = std::env::args().collect();
            if args.get(1).unwrap() == "help" {
                helpers::help::router(&args[2..]);
                return;
            }
            controller::run(command, &args[2..]);
        }
        None => {
            helpers::help::main_help::print_help();
        }
    }
}
