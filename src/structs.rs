use std::fmt;
use tmdb::model::*;

// Struct for movie entries
pub struct MovieEntry {
    pub title: String,
    pub id: u64,
    pub director: String,
    pub year: String,
    pub language: String,
    pub overview: String,
}

impl MovieEntry {
    // Create movie entry from results
    pub fn from(movie: SearchMovie) -> MovieEntry {
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
    pub fn rename_format(&self, mut format: String) -> String {
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

pub struct Language {
    pub short: String,
    pub long: String,
}

impl Language {
    // Create Language entries from &str pairs
    fn from(short: &str, long: &str) -> Language {
        Language {
            short: short.to_string(),
            long: long.to_string(),
        }
    }

    // Generate a vector of Language entries of all supported languages
    pub fn generate_list() -> Vec<Language> {
        let mut list = Vec::new();
        list.push(Language::from("en", "English"));
        list.push(Language::from("hi", "Hindi"));
        list.push(Language::from("bn", "Bengali"));
        list.push(Language::from("fr", "French"));
        list.push(Language::from("ja", "Japanese"));
        list.push(Language::from("de", "German"));
        list.push(Language::from("sp", "Spanish"));
        list.push(Language::from("none", "None"));
        list
    }
}

// Implement display trait for Language
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long)
    }
}
