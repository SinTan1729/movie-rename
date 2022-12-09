use load_file::{self, load_str};
use std::{collections::HashMap, env, fmt, fs, path::Path, process::exit};
use tmdb::{model::*, themoviedb::*};
use torrent_name_parser::Metadata;
use youchoose;

const VERSION: &str = "1.1.1";
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
    let (entries, settings) = process_args(args);

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

    // Iterate over entries
    for entry in entries {
        // Check if the file/directory exists on disk
        match settings["directory"] {
            // Normal file
            false => {
                if Path::new(entry.as_str()).is_file() == true {
                    // Process the filename for movie entries
                    process_file(&entry, &tmdb, pattern, settings["dry_run"]);
                } else {
                    eprintln!("The file {} wasn't found on disk, skipping...", entry);
                    continue;
                }
            }
            // Directory
            true => {
                if Path::new(entry.as_str()).is_dir() == true {
                    println!("Processing files inside the directory {}...", entry);
                    let mut movie_count = 0;
                    let mut movie_name = String::new();
                    if let Ok(files_in_dir) = fs::read_dir(entry.as_str()) {
                        for file in files_in_dir {
                            if file.is_ok() {
                                let (movie_name_temp, is_subtitle) = process_file(
                                    &format!("{}", file.unwrap().path().display()),
                                    &tmdb,
                                    pattern,
                                    settings["dry_run"],
                                );
                                if is_subtitle == false {
                                    movie_count += 1;
                                    movie_name = movie_name_temp;
                                }
                            }
                        }
                    } else {
                        eprintln!("There was an error accessing the directory {}!", entry);
                        continue;
                    }
                    if movie_count == 1 {
                        if entry == movie_name {
                            println!("[directory] {} already has correct name.", entry);
                        } else {
                            println!("[directory] {} -> {}", entry, movie_name);
                            if settings["dry_run"] == false {
                                fs::rename(entry, movie_name).expect("Unable to rename directory!");
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

// Function to process movie entries
fn process_file(filename: &String, tmdb: &TMDb, pattern: &str, dry_run: bool) -> (String, bool) {
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
    let mut menu = youchoose::Menu::new(movie_list.iter())
        .preview(display)
        .preview_label(file_base.to_string());
    let choice = menu.show()[0];

    let mut extension = metadata.extension().unwrap_or("").to_string();
    // Handle the case for subtitle files
    let mut is_subtitle = false;
    if ["srt", "ssa"].contains(&extension.as_str()) {
        let languages = Vec::from(["en", "hi", "bn", "de", "fr", "sp", "ja", "n/a"]);
        let mut lang_menu = youchoose::Menu::new(languages.iter());
        let lang_choice = lang_menu.show()[0];
        if languages[lang_choice] != "none" {
            extension = format!("{}.{}", languages[lang_choice], extension);
        }
        is_subtitle = true;
    }

    // Create the new name
    let new_name_base = movie_list[choice].rename_format(pattern.to_string());
    let mut new_name = String::from(new_name_base.clone());
    if extension != "" {
        new_name = format!("{}.{}", new_name, extension);
    }
    if parent != "".to_string() {
        new_name = format!("{}/{}", parent, new_name);
    }

    // Process the renaming
    if *filename == new_name {
        println!("[file] {} already has correct name.", filename);
    } else {
        println!("[file] {} -> {}", file_base, new_name);
        // Only do the rename of --dry-run isn't passed
        if dry_run == false {
            fs::rename(filename, new_name.as_str()).expect("Unable to rename file!");
        }
    }
    (new_name_base, is_subtitle)
}

// Display function for preview in menu
fn display(movie: &MovieEntry) -> String {
    let mut buffer = String::new();
    buffer.push_str(&format!("Title: {}\n", movie.title));
    buffer.push_str(&format!("Release year: {}\n", movie.year));
    buffer.push_str(&format!("Language: {}\n", movie.language));
    buffer.push_str(&format!("Director: {}\n", movie.director));
    buffer.push_str(&format!("TMDb ID: {}\n", movie.id));
    buffer.push_str(&format!("Overview: {}\n", movie.overview));
    buffer
}

// Function to process the passed arguments
fn process_args(mut args: Vec<String>) -> (Vec<String>, HashMap<&'static str, bool>) {
    // Remove the entry corresponding to the running process
    args.remove(0);
    let mut entries = Vec::new();
    let mut settings = HashMap::from([("dry_run", false), ("directory", false)]);
    for arg in args {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("  The expected syntax is:");
                println!(
                    "  movie_rename <filename(s)> [-n|--dry-run] [-d|--directory] [-v|--version]"
                );
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
                println!("Passing --directory assumes that the arguments are directory names, which contain exactly one movie and optionally subtitles.");
                println!("  Pass --help to get this again.");
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
            "--version" | "-v" => {
                println!("{}", VERSION);
                exit(0);
            }
            other => {
                if other.contains("-") {
                    eprintln!("Unknown argument passed: {}", other);
                } else {
                    entries.push(arg);
                }
            }
        }
    }
    (entries, settings)
}
