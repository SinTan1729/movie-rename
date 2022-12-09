use load_file::{self, load_str};
use std::{collections::HashMap, env, fmt, fs, path::Path, process::exit};
use tmdb::{model::*, themoviedb::*};
use torrent_name_parser::Metadata;
use youchoose;

// Struct for movie entries
struct MovieEntry {
    title: String,
    id: u64,
    director: String,
    year: String,
    language: String,
    overview: String,
}

impl MovieEntry {
    // Create movie entry from results
    fn from(movie: SearchMovie) -> MovieEntry {
        MovieEntry {
            title: movie.title,
            id: movie.id,
            director: String::from("N/A"),
            year: String::from(movie.release_date.split('-').next().unwrap_or("N/A")),
            language: movie.original_language,
            overview: movie.overview.unwrap_or(String::from("N/A")),
        }
    }

    // Generate desired filename from movie entry
    fn rename_format(&self, mut format: String) -> String {
        format = format.replace("{title}", self.title.as_str());
        if self.year.as_str() != "N/A" {
            format = format.replace("{year}", self.year.as_str());
        } else {
            format = format.replace("{year}", "");
        }
        if self.director.as_str() != "N/A" {
            format = format.replace("{director}", self.director.as_str());
        } else {
            format = format.replace("{director}", "");
        }
        format
    }
}

// Implement display trait for movie entries
impl fmt::Display for MovieEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

fn main() {
    // Read arguments from commandline
    let args: Vec<String> = env::args().collect();

    // Process the passed arguments
    let (filenames, settings) = process_args(args);

    // Try to read config file, or display error
    let mut config_file = env::var("XDG_CONFIG_HOME").unwrap_or("$HOME".to_string());
    if config_file == String::from("$HOME") {
        config_file = env::var("$HOME").unwrap();
    }
    config_file.push_str("/movie_rename.conf");
    let mut config = load_str!(config_file.as_str()).lines();
    let api_key = config.next().unwrap_or("");
    let pattern = config.next().unwrap_or("{title} ({year}) - {director}");

    if api_key == "" {
        eprintln!("Error reading the config file. Pass --help to see help.");
        exit(1);
    }

    // Create TMDb object for API calls
    let tmdb = TMDb {
        api_key: api_key,
        language: "en",
    };

    // Iterate over filenames
    for filename in filenames {
        // Check if the file/directory exists on disk
        match settings["directory"] {
            false => {
                if Path::new(filename.as_str()).is_file() == false {
                    eprintln!("{} wasn't found on disk, skipping...", filename);
                    continue;
                }
            }
            true => {
                if Path::new(filename.as_str()).is_dir() == false {
                    eprintln!("{} wasn't found on disk, skipping...", filename);
                    continue;
                }
            }
        }

        // Process the filename for movie entries
        process_file(&filename, &tmdb, pattern, settings["dry_run"]);
    }
}

// Function to process movie entries
fn process_file(filename: &String, tmdb: &TMDb, pattern: &str, dry_run: bool) {
    // Parse the filename for metadata
    let metadata = Metadata::from(filename.as_str()).expect("Could not parse filename");
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
        eprintln!("There was an error while searching {}", filename);
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
        eprintln!("Could not find any entries matching {}", filename);
        return;
    }

    // Choose from the possible entries
    let mut menu = youchoose::Menu::new(movie_list.iter())
        .preview(display)
        .preview_label(filename.to_string());
    let choice = menu.show()[0];

    let mut extension = metadata.extension().unwrap_or("").to_string();
    // Handle the case for subtitle files
    if ["srt", "ssa"].contains(&extension.as_str()) {
        let languages = Vec::from(["en", "hi", "bn", "de", "fr", "sp", "ja", "n/a"]);
        let mut lang_menu = youchoose::Menu::new(languages.iter());
        let lang_choice = lang_menu.show()[0];
        if languages[lang_choice] != "none" {
            extension = format!("{}.{}", languages[lang_choice], extension);
        }
    }

    // Create the new name
    let mut new_name_vec = vec![
        movie_list[choice].rename_format(pattern.to_string()),
        extension,
    ];
    new_name_vec.retain(|x| !x.is_empty());
    let new_name = new_name_vec.join(".");

    // Process the renaming
    if *filename == new_name {
        println!("{} already has correct name.", filename);
    } else {
        println!("{} -> {}", filename, new_name);
        // Only do the rename of --dry-run isn't passed
        if dry_run == false {
            println!("Renaming...");
            fs::rename(filename, new_name).expect("Unable to rename file.");
        }
    }
}

// Display function for preview in menu
fn display(movie: &MovieEntry) -> String {
    let mut buffer = String::new();
    buffer.push_str(&format!("Title: {}\n", movie.title));
    buffer.push_str(&format!("Release year: {}\n", movie.year));
    buffer.push_str(&format!("Language: {}\n", movie.language));
    buffer.push_str(&format!("Director: {}\n", movie.title));
    buffer.push_str(&format!("TMDb ID: {}\n", movie.id));
    buffer.push_str(&format!("Overview: {}\n", movie.overview));
    buffer
}

// Function to process the passed arguments
fn process_args(mut args: Vec<String>) -> (Vec<String>, HashMap<&'static str, bool>) {
    // Remove the entry corresponding to the running process
    args.remove(0);
    let mut filenames = Vec::new();
    let mut settings = HashMap::from([("dry_run", false), ("directory", false)]);
    for arg in args {
        match arg.as_str() {
            "--help" => {
                println!("  The expected syntax is:");
                println!("  movie_rename <filename(s)> [--dry-run]");
                println!(
                "  There needs to be a config file names movie_rename.conf in your $XDG_CONFIG_HOME."
                );
                println!("  It should consist of two lines. The first line should have your TMDb API key.");
                println!(
                    "  The second line should have a pattern, that will be used for the rename."
                );
                println!("  In the pattern, the variables need to be enclosed in {{}}, the supported variables are `title`, `year` and `director`.");
                println!(
                "  Default pattern is `{{title}} ({{year}}) - {{director}}`. Extension is always kept."
                );
                println!("  Pass --help to get this again.");
                exit(0);
            }
            "--dry-run" => {
                println!("Doing a dry run...");
                settings.entry("dry_run").and_modify(|x| *x = true);
            }
            "--directory" => {
                println!("Running in directory mode...");
                settings.entry("directory").and_modify(|x| *x = true);
            }
            _ => {
                filenames.push(arg);
            }
        }
    }
    (filenames, settings)
}
