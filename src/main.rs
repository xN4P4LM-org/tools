use crate::{helpers::config, models::app_config::AppConfigFile};
use once_cell::sync::Lazy;

/// This is the main file for the program
mod controller;
mod helpers;
mod models;

pub static CONFIG: Lazy<AppConfigFile> = Lazy::new(config::app_config::get_config);

/// ## main()
/// This function is the entry point for the program
fn main() {

    // // switch statement based on the first argument and pass the rest of the arguments to the controller
    // match std::env::args().nth(1) {
    //     Some(command) => {
    //         let args: Vec<String> = std::env::args().collect();
    //         if args.get(1).unwrap() == "help" {
    //             helpers::help::router(&args[2..]);
    //             return;
    //         }
    //         controller::run(command, &args[2..]);
    //     }
    //     None => {
    //         helpers::help::main_help::print_help();
    //     }
    // }
}
