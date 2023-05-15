use std::fmt;
use tmdb_api::movie::MovieShort;

// Struct for movie entries
pub struct MovieEntry {
    pub title: String,
    pub id: u64,
    pub director: String,
    pub year: String,
    pub language: String,
}

impl MovieEntry {
    // Create movie entry from results
    pub fn from(movie: MovieShort) -> MovieEntry {
        MovieEntry {
            title: movie.inner.title,
            id: movie.inner.id,
            director: String::from("N/A"),
            year: match movie.inner.release_date {
                Some(date) => date.format("%Y").to_string(),
                _ => "N/A".to_string(),
            },
            language: get_long_lang(movie.inner.original_language.as_str()),
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
        let mut buffer = String::new();
        buffer.push_str(&format!("{} ", self.title));
        buffer.push_str(&format!("({}), ", self.year));
        buffer.push_str(&format!(
            "Language: {}, ",
            get_long_lang(self.language.as_str())
        ));
        buffer.push_str(&format!("Directed by: {}, ", self.director));
        buffer.push_str(&format!("TMDB ID: {}", self.id));
        // buffer.push_str(&format!("Synopsis: {}", self.overview));
        write!(f, "{}", buffer)
    }
}

pub struct Language {
    pub short: String,
    pub long: String,
}

impl Language {
    // Generate a vector of Language entries of all supported languages
    pub fn generate_list() -> Vec<Language> {
        let mut list = Vec::new();
        for lang in ["en", "hi", "bn", "fr", "ja", "de", "sp", "none"] {
            list.push(Language {
                short: lang.to_string(),
                long: get_long_lang(lang),
            });
        }
        list
    }
}

// Implement display trait for Language
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.long)
    }
}

// Get long name of a language
pub fn get_long_lang(short: &str) -> String {
    let long = match short {
        "en" => "English",
        "hi" => "Hindi",
        "bn" => "Bengali",
        "fr" => "French",
        "ja" => "Japanese",
        "de" => "German",
        "sp" => "Spanish",
        "none" => "None",
        other => other,
    };
    long.to_string()
}
