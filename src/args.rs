use clap::{arg, command, ArgAction, Command, ValueHint};
use std::{collections::HashMap, process::exit};

// Bare command generation function to help with autocompletion
pub fn get_command() -> Command {
    command!()
        .name("movie-rename")
        .author("Sayantan Santra <sayantan.santra@gmail.com>")
        .about("A simple tool to rename movies, written in Rust.")
        .arg(arg!(-d --directory "Run in directory mode").action(ArgAction::SetTrue))
        .arg(arg!(-n --"dry-run" "Do a dry run").action(ArgAction::SetTrue))
        .arg(arg!(-l --"i-feel-lucky" "Always choose the first option").action(ArgAction::SetTrue))
        .arg(arg!(-i --"tmdb-id" <INTEGER> "Use a given TMDB ID. Does not work with directory mode."))
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
            "{before-help}{name} {version}\n{about}\nGPLv3 (c) {author}
                \n{usage-heading}\n{usage}\n\n{all-args}{after-help}",
        )
}

// Function to process the passed arguments
pub fn process_args() -> (Vec<String>, Option<u64>, HashMap<String, bool>) {
    let matches = get_command().get_matches();

    // Generate the settings HashMap from read flags
    let mut settings = HashMap::new();
    for id in matches.ids().map(|x| x.as_str()) {
        if !["entries", "tmdb-id"].contains(&id) {
            settings.insert(id.to_string(), matches.get_flag(id));
        }
    }

    let tmdb_id = matches
        .get_one::<String>("tmdb-id")
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|&n| n > 0);

    if tmdb_id.is_some() && settings["directory"] {
        eprintln!("Directory mode does not support providing TMDB ID.");
        exit(1);
    }

    // Every unmatched argument should be treated as a file entry
    let entries: Vec<String> = matches
        .get_many::<String>("entries")
        .expect("No entries provided!")
        .cloned()
        .collect();

    (entries, tmdb_id, settings)
}
