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
