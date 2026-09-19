use std::process::exit;

use crate::cli::get_command;
use crate::structs::Arguments;

// Function to process the passed arguments
pub fn process_args() -> Arguments {
    let matches = get_command().get_matches();

    // Generate the settings HashMap from read flags
    let mut args = Arguments {
        directory_mode: false,
        dry_run: false,
        i_feel_lucky: false,
        tmdb_id: None,
        items: Vec::new(),
    };
    for id in matches.ids().map(|x| x.as_str()) {
        match id {
            "directory" => args.directory_mode = matches.get_flag(id),
            "dry-run" => args.dry_run = matches.get_flag(id),
            "i-feel-lucky" => args.i_feel_lucky = matches.get_flag(id),
            _ => {}
        }
    }

    args.tmdb_id = matches
        .get_one::<String>("tmdb-id")
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|&n| n > 0);

    if args.tmdb_id.is_some() && args.directory_mode {
        eprintln!("Directory mode does not support providing TMDB ID.");
        exit(1);
    }

    // Every unmatched argument should be treated as a file entry
    args.items = matches
        .get_many::<String>("entries")
        .expect("No entries provided!")
        .cloned()
        .collect();

    args
}
