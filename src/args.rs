use clap::{arg, command, ArgAction, Command, ValueHint};
use std::collections::HashMap;

// Bare command generation function to help with autocompletion
pub fn get_command() -> Command {
    command!()
        .name("movie-rename")
        .author("Sayantan Santra <sayantan.santra@gmail.com>")
        .about("A simple tool to rename movies, written in Rust.")
        .arg(arg!(-d --directory "Run in directory mode").action(ArgAction::SetTrue))
        .arg(arg!(-n --"dry-run" "Do a dry run").action(ArgAction::SetTrue))
        .arg(arg!(-l --"i-feel-lucky" "Always choose the first option").action(ArgAction::SetTrue))
        .arg(
            arg!([entries] "The files/directories to be processed")
                .trailing_var_arg(true)
                .num_args(1..)
                .value_hint(ValueHint::AnyPath)
                .required(true),
        )
        // Use -v instead of -V for version
        .disable_version_flag(true)
        .arg(arg!(-v --version "Print version").action(ArgAction::Version))
        .arg_required_else_help(true)
        .help_template(
            "{before-help}{name} {version}\n{about}\nMIT (c) {author}
                \n{usage-heading}\n{usage}\n\n{all-args}{after-help}",
        )
}

// Function to process the passed arguments
pub fn process_args() -> (Vec<String>, HashMap<String, bool>) {
    let matches = get_command().get_matches();

    // Generate the settings HashMap from read flags
    let mut settings = HashMap::new();
    for id in matches.ids().map(|x| x.as_str()) {
        if id != "entries" {
            settings.insert(id.to_string(), matches.get_flag(id));
        }
    }

    // Every unmatched argument should be treated as a file entry
    let entries: Vec<String> = matches
        .get_many::<String>("entries")
        .expect("No entries provided!")
        .cloned()
        .collect();

    (entries, settings)
}
