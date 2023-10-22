//multiline variable for the main help file
pub const MAIN_HELP: &str = r#"
Usage: 
    tools [command] [subcommand] [arguments]
    
Arguments:
    help, -h, --help        Prints this help message
    version, --version      Prints the version of the program
    -v(#)                   Sets the verbosity level of the program (0-5) (default: 3) - (Not implemented yet)
    -q                      Sets the verbosity level to 0 (same as -v0) - (Not implemented yet)

Commands:
    down      Stops the project - (Not implemented yet)
    init      Initializes the project
    up        Starts the project - (Not implemented yet)
    watch     Starts the project and watches for changes - (Not implemented yet)
    status    Prints the status of the project - (Not implemented yet)

For more information on a command, run `tools help [command]

"#;

pub fn print_help() {
    println!("{}", MAIN_HELP);
}
