//multiline variable for the main help file
pub const INIT_HELP: &str = r#"
Usage: 
    tools init [OPTIONS] [Subcommand]

Options:
    -h, --help      Prints help information

Subcommands:
    none            Initializes the project
    new             Creates a new project
    rebuild         Rebuilds the project
"#;

pub fn print_help() {
    println!("{}", INIT_HELP);
}
