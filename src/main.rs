use load_file::{self, load_str};
use std::{env, fmt};
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

    fn rename_format(&self) -> String {
        format!("{} ({}) - {}", self.title, self.year, self.director)
    }
}

impl fmt::Display for MovieEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let key_file = &args[1];
    let filename = &args[2];
    let api_key = load_str!(key_file);

    let tmdb = TMDb {
        api_key: api_key,
        language: "en",
    };

    let metadata = Metadata::from(filename).unwrap();
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
    let mut new_name = vec![
        movie_list[choice].rename_format(),
        metadata.extension().unwrap_or("").to_string(),
    ];
    new_name.retain(|x| !x.is_empty());
    println!("{}", new_name.join("."));
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
