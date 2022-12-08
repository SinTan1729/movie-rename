use load_file::{self, load_str};
use std::{env, fmt, fs, process::exit};
use tmdb::{model::*, themoviedb::*};
use torrent_name_parser::Metadata;
use youchoose;

struct MovieEntry {
    title: String,
    id: u64,
    director: String,
    year: String,
    language: String,
    overview: String,
}

impl MovieEntry {
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

    fn rename_format(&self, mut format: String) -> String {
        format = format.replace("{title}", self.title.as_str());
        format = format.replace("{year}", self.year.as_str());
        format = format.replace("{director}", self.director.as_str());
        format
    }
}

impl fmt::Display for MovieEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let filenames: Vec<String> = args.collect();

    if filenames.contains(&"--help".to_string()) {
        println!("The expected syntax is:");
        println!("movie_rename <filename(s)> [--dry-run]");
        println!(
            "There needs to be a config file names movie_rename.conf in your $XDG_CONFIG_HOME."
        );
        println!("It should consist of two lines. The first line should have your TMDb API key.");
        println!("The second line should have a pattern, that will be used for the rename.");
        println!("In the pattern, the variables need to be enclosed in {{}}, the supported variables are `title`, `year` and `director`.");
        println!(
            "Extension is always kept. Default pattern is `{{title}} ({{year}}) - {{director}}`"
        );
        exit(0);
    }

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

    let tmdb = TMDb {
        api_key: api_key,
        language: "en",
    };

    let mut dry_run = false;

    if filenames.contains(&"--dry-run".to_string()) {
        println!("Doing a dry run.");
        dry_run = true;
    }

    for filename in filenames {
        if filename == "--dry-run".to_string() {
            continue;
        }

        let metadata = Metadata::from(filename.as_str()).unwrap();
        let results = tmdb
            .search()
            .title(metadata.title())
            .year(metadata.year().unwrap() as u64)
            .execute()
            .unwrap()
            .results;

        let mut movie_list: Vec<MovieEntry> = Vec::new();

        for result in results {
            let mut movie_details = MovieEntry::from(result);
            let with_credits: Result<Movie, _> =
                tmdb.fetch().id(movie_details.id).append_credits().execute();
            if let Ok(movie) = with_credits {
                match movie.credits {
                    Some(cre) => {
                        let mut directors = cre.crew;
                        directors.retain(|x| x.job == "Director");
                        for person in directors {
                            movie_details.director = person.name;
                        }
                    }
                    None => {}
                }
            }
            movie_list.push(movie_details);
        }

        let mut menu = youchoose::Menu::new(movie_list.iter())
            .preview(display)
            .preview_label(filename.to_string());
        let choice = menu.show()[0];

        let mut extension = metadata.extension().unwrap_or("").to_string();
        if ["srt", "ssa"].contains(&extension.as_str()) {
            let languages = Vec::from(["en", "hi", "bn", "de", "fr", "sp", "ja", "n/a"]);
            let mut lang_menu = youchoose::Menu::new(languages.iter());
            let lang_choice = lang_menu.show()[0];
            if languages[lang_choice] != "none" {
                extension = format!("{}.{}", languages[lang_choice], extension);
            }
        }

        let mut new_name_vec = vec![
            movie_list[choice].rename_format(pattern.to_string()),
            extension,
        ];
        new_name_vec.retain(|x| !x.is_empty());
        let new_name = new_name_vec.join(".");
        if filename == new_name {
            println!("{} already has correct name.", filename);
        } else {
            println!("{} -> {}", filename, new_name);
            if dry_run == false {
                println!("Doing the actual rename.");
                fs::rename(filename, new_name).expect("Unable to rename file.");
            }
        }
    }
}

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
