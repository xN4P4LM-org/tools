pub mod init_help;
pub mod main_help;

pub fn router(args: &[String]) {
    match args.get(0) {
        Some(arg0) => match arg0.as_str() {
            "init" => init_help::print_help(),
            _ => main_help::print_help(),
        },
        None => {
            main_help::print_help();
        }
    }
}

// ### help_not_implemented_yet()
// This function prints a message that the help command is not implemented yet
//
// ### Returns
// This function does not return anything
pub fn help_not_implemented_yet() {
    println!("Help is not implemented yet");
}
