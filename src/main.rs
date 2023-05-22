use load_file::{self, load_str};
use std::{collections::HashMap, env, fs, path::Path, process::exit};
use tmdb_api::Client;

// Import all the modules
mod functions;
use functions::{process_args, process_file};
mod structs;

#[tokio::main]
async fn main() {
    // Read arguments from commandline
    let args: Vec<String> = env::args().collect();

    // Process the passed arguments
    let (entries, settings) = process_args(args);

    // Try to read config file, or display error
    let mut config_file = env::var("XDG_CONFIG_HOME").unwrap_or("$HOME".to_string());
    if config_file == *"$HOME" {
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
    let tmdb = Client::new(api_key.to_string());

    // Iterate over entries
    for entry in entries {
        // Check if the file/directory exists on disk and run necessary commands
        match settings["directory"] {
            // Normal file
            false => {
                if Path::new(entry.as_str()).is_file() {
                    // Process the filename for movie entries
                    process_file(&entry, &tmdb, pattern, settings["dry_run"], None).await;
                } else {
                    eprintln!("The file {} wasn't found on disk, skipping...", entry);
                    continue;
                }
            }
            // Directory
            true => {
                if Path::new(entry.as_str()).is_dir() {
                    println!("Processing files inside the directory {}...", entry);
                    let mut movie_list = HashMap::new();

                    if let Ok(files_in_dir) = fs::read_dir(entry.as_str()) {
                        for file in files_in_dir {
                            if file.is_ok() {
                                let filename = file.unwrap().path().display().to_string();
                                let (filename_without_ext, movie_name_temp, add_to_list) =
                                    process_file(
                                        &filename,
                                        &tmdb,
                                        pattern,
                                        settings["dry_run"],
                                        Some(&movie_list),
                                    )
                                    .await;

                                // if movie_name_temp.is_empty() {
                                //     continue;
                                // }

                                if add_to_list {
                                    movie_list.insert(filename_without_ext, movie_name_temp);
                                }
                            }
                        }
                    } else {
                        eprintln!("There was an error accessing the directory {}!", entry);
                        continue;
                    }
                    if movie_list.len() == 1 {
                        let entry_clean = entry.trim_end_matches('/');
                        let movie_name = movie_list.into_values().next().unwrap();

                        if entry_clean == movie_name {
                            println!("[directory] '{}' already has correct name.", entry_clean);
                        } else {
                            println!("[directory] '{}' -> '{}'", entry_clean, movie_name);
                            if !settings["dry_run"] {
                                if !Path::new(movie_name.as_str()).is_dir() {
                                    fs::rename(entry, movie_name)
                                        .expect("Unable to rename directory!");
                                } else {
                                    eprintln!("Destination directory already exists, skipping...");
                                }
                            }
                        }
                    } else {
                        eprintln!("Could not determine how to rename the directory {}!", entry);
                    }
                } else {
                    eprintln!("The directory {} wasn't found on disk, skipping...", entry);
                    continue;
                }
            }
        }
    }
}
