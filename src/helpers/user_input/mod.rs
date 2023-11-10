/// ## get_user_input(prompt: &str) -> String
/// This function gets user input from stdin and returns it as a String
/// it also has an optional argument to normalize the input
///
/// ### Arguments
/// - prompt: &str - The prompt to display to the user
/// - normalize (optional) - bool - Whether or not to normalize the input
///
/// ### Returns
/// - String - The user input
pub fn get_user_input(prompt: &str, normalize: Option<bool>) -> String {
    // normalize the prompt
    let normalized_prompt = normalize_prompt(prompt);

    // print the prompt
    print!("{}", normalized_prompt);

    // create a new string to store the user input
    let mut input = String::new();

    // read the user input
    std::io::stdin().read_line(&mut input).unwrap();

    // check if normalize is provided, and then what it option was set too
    match normalize {
        Some(true) => {
            // normalize the input
            input = input.trim().to_string().to_ascii_lowercase();
        }
        Some(false) => {
            input = input.trim().to_string();
        }
        None => {
            input = input.trim().to_string();
        }
    }

    // return the user input
    input
}

/// ## get_user_input_with_options(prompt: &str, options: &[&str]) -> bool
/// This function gets user input from stdin and returns it as a String
/// if the input matches one of the options
///
/// This function is case insensitive and normalizes the input to lowercase ASCII
///
/// ### Arguments
/// - prompt: &str - The prompt to display to the user
/// - options: &[&str] - A list of ASCII options to match the user input against
///
/// ### Returns
/// - bool - true if the user input matches one of the options, false otherwise
pub fn get_user_input_with_options(prompt: &str, options: &[&str]) -> bool {
    // if prompt is empty, panic
    if prompt.is_empty() {
        panic!("Prompt cannot be empty for get_user_input_with_options");
    }

    // normalize the prompt
    let normalized_prompt = normalize_prompt(prompt);

    // print the normalized_prompt
    print!("{}", normalized_prompt);

    // create a new string to store the user input
    let mut input = String::new();

    // read the user input
    std::io::stdin().read_line(&mut input).unwrap();

    // normalize the input
    input = input.trim().to_string().to_ascii_lowercase();

    // check if the user input matches one of the options
    if options.contains(&input.trim()) {
        // return true
        true
    } else {
        // return false
        false
    }
}

/// ## normalize_prompt(prompt: &str) -> String
/// This function normalizes the prompt by adding a colon and a space
/// to the end of the prompt if it is not already there
///
/// ### Arguments
/// - prompt: &str - The prompt to normalize
///
/// ### Returns
/// - String - The normalized prompt
pub fn normalize_prompt(prompt: &str) -> String {
    // check if prompt is empty and set it to a default value if it is
    let local_prompt = if prompt.is_empty() {
        "Enter input: "
    } else {
        prompt // return the prompt
    };

    // trim tailing whitespace
    let local_prompt = local_prompt.trim();

    // Check 1: if the prompt ends with a letter or number add a colon and a space
    // Check 2: if the prompt ends with a character that is not colon, remove it and add a colon and a space
    // Check 3: if the prompt ends with a colon and a space, do nothing
    if local_prompt.ends_with(|c: char| c.is_ascii_alphanumeric()) {
        // add a colon and a space
        format!("{}: ", local_prompt)
    } else if local_prompt.ends_with(|c: char| !c.is_ascii_alphanumeric() && c != ':') {
        // remove the last character and add a colon and a space
        format!(
            "{}: ",
            local_prompt.trim_end_matches(|c: char| !c.is_ascii_alphanumeric())
        )
    } else if local_prompt.ends_with(": ") {
        // do nothing
        local_prompt.to_string()
    } else {
        // add a colon and a space
        format!("{}: ", local_prompt)
    }
}

/// ## ask_user_yes_or_no(prompt: &str) -> bool
/// This function asks the user a yes or no question and returns true if the user
/// answers yes and false if the user answers no
/// This function is case insensitive and normalizes the input to lowercase ASCII
/// This function will keep asking the user for input until they answer yes or no
///
/// ### Arguments
/// - prompt: &str - The prompt to display to the user
///
/// ### Returns
/// - bool - true if the user answers yes, false if the user answers no
pub fn ask_user_yes_or_no(prompt: &str) -> bool {
    // create a new string to store the user input
    let mut input = String::new();

    // loop until the user answers yes or no
    loop {
        // get the user input
        input = get_user_input(prompt, Some(true));

        // check if the user input is yes or no
        if input == "yes" || input == "no" {
            // break out of the loop
            break;
        } else {
            // print an error message
            println!("Please enter yes or no");
        }
    }

    // check if the user input is yes
    if input == "yes" {
        // return true
        true
    } else {
        // return false
        false
    }
}
