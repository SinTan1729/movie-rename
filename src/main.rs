// SPDX-FileCopyrightText: 2026 Sayantan Santra <sayantan.santra689@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use load_file::{self, load_str};
use std::{collections::HashMap, env, fs, path::Path, process::exit};
use tmdb_api::client::{Client, reqwest::ReqwestExecutor};

// Import all the modules
mod functions;
use functions::process_file;
mod args;
mod cli;
mod structs;

#[tokio::main]
async fn main() {
    // Process the passed arguments
    let args = args::process_args();

    // Print some message when flags are set.
    if args.dry_run {
        println!("Doing a dry run. No files will be modified.")
    }
    if args.directory_mode {
        println!("Running in directory mode...")
    }
    if args.i_feel_lucky {
        println!("Automatically selecting the first entry...")
    }

    // Try to read config file, or display error
    let mut config_file = env::var("XDG_CONFIG_HOME").unwrap_or(String::from("$HOME"));
    if config_file == "$HOME" {
        config_file = env::var("$HOME").unwrap();
        config_file.push_str("/.config");
    }
    config_file.push_str("/movie-rename/config");

    if !Path::new(config_file.as_str()).is_file() {
        eprintln!("Error reading the config file. Pass --help to see help.");
        exit(2);
    }

    let mut config = load_str!(config_file.as_str()).lines();
    let api_key = config.next().unwrap_or("");
    let pattern = config.next().unwrap_or("{title} ({year}) - {director}");

    if api_key.is_empty() {
        eprintln!("Could not read the API key. Pass --help to see help.");
        exit(2);
    }

    // Create TMDb object for API calls
    let tmdb = Client::<ReqwestExecutor>::new(String::from(api_key));

    // Iterate over entries
    for item in args.items.iter() {
        // Check if the file/directory exists on disk and run necessary commands
        match args.directory_mode {
            // Normal file
            false => {
                if Path::new(item.as_str()).is_file() {
                    // Process the filename for movie entries
                    process_file(&item, &tmdb, &args, pattern, None).await;
                } else {
                    eprintln!("The file {item} wasn't found on disk, skipping...");
                    continue;
                }
            }
            // Directory
            true => {
                if Path::new(item.as_str()).is_dir() {
                    println!("Processing files inside the directory {item}...");
                    let mut movie_list = HashMap::new();

                    if let Ok(files_in_dir) = fs::read_dir(item.as_str()) {
                        let filename_list: Vec<_> = files_in_dir
                            .filter(|f| f.is_ok())
                            .map(|f| f.unwrap().path().display().to_string())
                            .collect();
                        for filename in filename_list {
                            let (filename_without_ext, movie_name_temp, add_to_list) =
                                process_file(&filename, &tmdb, &args, pattern, Some(&movie_list))
                                    .await;

                            if add_to_list {
                                movie_list.insert(filename_without_ext, movie_name_temp);
                            }
                        }
                    } else {
                        eprintln!("There was an error accessing the directory {item}!");
                        continue;
                    }
                    if movie_list.len() == 1 {
                        let entry_clean = item.trim_end_matches('/');
                        let movie_name = movie_list.into_values().next().unwrap();

                        // If the file was ignored, exit
                        match movie_name {
                            None => {
                                eprintln!("Not renaming directory as only movie was skipped.");
                            }

                            Some(name) => {
                                if entry_clean == name {
                                    println!(
                                        "[directory] '{entry_clean}' already has correct name."
                                    );
                                } else {
                                    println!("[directory] '{entry_clean}' -> '{name}'",);
                                    if !args.dry_run {
                                        if !Path::new(name.as_str()).is_dir() {
                                            fs::rename(item, name)
                                                .expect("Unable to rename directory!");
                                        } else {
                                            eprintln!(
                                                "Destination directory already exists, skipping..."
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        eprintln!("Could not determine how to rename the directory {item}!");
                    }
                } else {
                    eprintln!("The directory {item} wasn't found on disk, skipping...");
                    continue;
                }
            }
        }
    }
}
