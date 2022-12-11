use inquire::{
    ui::{Color, IndexPrefix, RenderConfig, Styled},
    Select,
};
use std::{collections::HashMap, fs, path::Path, process::exit};
use tmdb::{model::*, themoviedb::*};
use torrent_name_parser::Metadata;

use crate::structs::{get_long_lang, Language, MovieEntry};

// Get the version from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Function to process movie entries
pub fn process_file(
    filename: &String,
    tmdb: &TMDb,
    pattern: &str,
    dry_run: bool,
) -> (String, bool) {
    // Set RenderConfig for the menu items
    inquire::set_global_render_config(get_render_config());

    // Get the basename
    let mut file_base = String::from(filename);
    let mut parent = String::from("");
    match filename.rsplit_once("/") {
        Some(parts) => {
            parent = parts.0.to_string();
            file_base = parts.1.to_string();
        }
        None => {}
    }

    // Parse the filename for metadata
    let metadata = Metadata::from(file_base.as_str()).expect("Could not parse filename!");
    // Search using the TMDb API
    let mut search = tmdb.search();
    search.title(metadata.title());

    // Check if year is present in filename
    if let Some(year) = metadata.year() {
        search.year(year as u64);
    }

    let mut results = Vec::new();
    if let Ok(search_results) = search.execute() {
        results = search_results.results;
    } else {
        eprintln!("There was an error while searching {}!", filename);
    }

    let mut movie_list: Vec<MovieEntry> = Vec::new();
    // Create movie entry from the result
    for result in results {
        let mut movie_details = MovieEntry::from(result);
        // Get director's name, if needed
        if pattern.contains("{director}") {
            let with_credits: Result<Movie, _> =
                tmdb.fetch().id(movie_details.id).append_credits().execute();
            if let Ok(movie) = with_credits {
                if let Some(cre) = movie.credits {
                    let mut directors = cre.crew;
                    directors.retain(|x| x.job == "Director");
                    for person in directors {
                        movie_details.director = person.name;
                    }
                }
            }
        }
        movie_list.push(movie_details);
    }

    // If nothing is found, skip
    if movie_list.len() == 0 {
        eprintln!("Could not find any entries matching {}!", filename);
        return ("".to_string(), true);
    }

    // Choose from the possible entries
    let choice = Select::new(
        format!("Possible choices for {}:", file_base).as_str(),
        movie_list,
    )
    .prompt()
    .expect("Invalid choice!");

    let mut extension = metadata.extension().unwrap_or("").to_string();
    // Handle the case for subtitle files
    let mut is_subtitle = false;
    if ["srt", "ssa"].contains(&extension.as_str()) {
        // Try to detect if there's already language info in the filename, else ask user to choose
        let filename_parts: Vec<&str> = filename.rsplit(".").collect();
        if filename_parts.len() >= 3 && filename_parts[1].len() == 2 {
            println!(
                "Keeping language {} as detected in the subtitle file's extension...",
                get_long_lang(filename_parts[1])
            );
            extension = format!("{}.{}", filename_parts[1], extension);
        } else {
            let lang_list = Language::generate_list();
            let lang_choice = Select::new("Choose the language for the subtitle file:", lang_list)
                .prompt()
                .expect("Invalid choice!");
            if lang_choice.short != "none".to_string() {
                extension = format!("{}.{}", lang_choice.short, extension);
            }
        }
        is_subtitle = true;
    }

    // Create the new name
    let new_name_base = choice.rename_format(pattern.to_string());
    let mut new_name_with_ext = new_name_base.clone();
    if extension != "" {
        new_name_with_ext = format!("{}.{}", new_name_with_ext, extension);
    }
    let mut new_name = String::from(new_name_with_ext.clone());
    if parent != "".to_string() {
        new_name = format!("{}/{}", parent, new_name);
    }

    // Process the renaming
    if *filename == new_name {
        println!("[file] '{}' already has correct name.", file_base);
    } else {
        println!("[file] '{}' -> '{}'", file_base, new_name_with_ext);
        // Only do the rename of --dry-run isn't passed
        if dry_run == false {
            if Path::new(new_name.as_str()).is_file() == false {
                fs::rename(filename, new_name.as_str()).expect("Unable to rename file!");
            } else {
                eprintln!("Destination file already exists, skipping...");
            }
        }
    }
    (new_name_base, is_subtitle)
}

// Function to process the passed arguments
pub fn process_args(mut args: Vec<String>) -> (Vec<String>, HashMap<&'static str, bool>) {
    // Remove the entry corresponding to the running process
    args.remove(0);
    let mut entries = Vec::new();
    let mut settings = HashMap::from([("dry_run", false), ("directory", false)]);
    for arg in args {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("  The expected syntax is:");
                println!(
                    "  movie-rename <filename(s)> [-n|--dry-run] [-d|--directory] [-v|--version]"
                );
                println!(
                "  There needs to be a config file named config in the $XDG_CONFIG_HOME/movie-rename/ directory."
                );
                println!("  It should consist of two lines. The first line should have your TMDb API key.");
                println!(
                    "  The second line should have a pattern, that will be used for the rename."
                );
                println!("  In the pattern, the variables need to be enclosed in {{}}, the supported variables are `title`, `year` and `director`.");
                println!(
                "  Default pattern is `{{title}} ({{year}}) - {{director}}`. Extension is always kept."
                );
                println!("  Passing --directory or -d assumes that the arguments are directory names, which contain exactly one movie and optionally subtitles.");
                println!("  Passing --dry-run or -n does a dry tun and only prints out the new names, without actually doing anything.");
                println!("  Passing -nd or -dn does a dry run in directory mode.");
                println!("  Passing --version or -v shows version and exits.");
                println!("  Pass --help to get this again.");
                exit(0);
            }
            "--version" | "-v" => {
                println!("movie-rename {}", VERSION);
                exit(0);
            }
            "--dry-run" | "-n" => {
                println!("Doing a dry run...");
                settings.entry("dry_run").and_modify(|x| *x = true);
            }
            "--directory" | "-d" => {
                println!("Running in directory mode...");
                settings.entry("directory").and_modify(|x| *x = true);
            }
            "-nd" | "-dn" => {
                println!("Doing a dry run in directory mode...");
                settings.entry("dry_run").and_modify(|x| *x = true);
                settings.entry("directory").and_modify(|x| *x = true);
            }
            other => {
                if other.starts_with("-") {
                    eprintln!("Unknown argument passed: {}", other);
                    exit(1);
                } else {
                    entries.push(arg);
                }
            }
        }
    }
    (entries, settings)
}
// RenderConfig for the menu items
fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();
    render_config.option_index_prefix = IndexPrefix::Simple;

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config
}
