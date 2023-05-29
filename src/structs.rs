use std::fmt;
use tmdb_api::movie::MovieShort;

// Struct for movie entries
pub struct MovieEntry {
    pub title: String,
    pub id: u64,
    pub director: Option<String>,
    pub year: Option<String>,
    pub language: String,
}

impl MovieEntry {
    // Create movie entry from results
    pub fn from(movie: MovieShort) -> MovieEntry {
        MovieEntry {
            title: movie.inner.title,
            id: movie.inner.id,
            director: None,
            year: movie
                .inner
                .release_date
                .map(|date| date.format("%Y").to_string()),
            language: get_long_lang(movie.inner.original_language.as_str()),
        }
    }

    // Generate desired filename from movie entry
    pub fn rename_format(&self, mut format: String) -> String {
        // Try to sanitize the title to avoid some characters
        let mut title = self.title.clone();
        title = sanitize(title);
        title.truncate(159);
        format = format.replace("{title}", title.as_str());

        format = match &self.year {
            Some(year) => format.replace("{year}", year.as_str()),
            None => format.replace("{year}", ""),
        };

        format = match &self.director {
            Some(name) => {
                // Try to sanitize the director's name to avoid some characters
                let mut director = name.clone();
                director = sanitize(director);
                director.truncate(63);
                format.replace("{director}", director.as_str())
            }
            None => format.replace("{director}", ""),
        };

        // Try to clean extra spaces and such
        format = format.trim_matches(|c| "- ".contains(c)).to_string();
        while format.contains("- -") {
            format = format.replace("- -", "-");
        }

        format
    }
}

// Implement display trait for movie entries
impl fmt::Display for MovieEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        buffer.push_str(&format!("{} ", self.title));

        if self.year.is_some() {
            buffer.push_str(&format!("({}), ", self.year.as_ref().unwrap()));
        }
        buffer.push_str(&format!(
            "Language: {}, ",
            get_long_lang(self.language.as_str())
        ));

        if self.director.is_some() {
            buffer.push_str(&format!(
                "Directed by: {}, ",
                self.director.as_ref().unwrap()
            ));
        }
        buffer.push_str(&format!("TMDB ID: {}", self.id));
        // buffer.push_str(&format!("Synopsis: {}", self.overview));
        write!(f, "{buffer}")
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
                short: String::from(lang),
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
        "fa" => "Persian",
        "none" => "None",
        other => other,
    };
    String::from(long)
}

// Sanitize filename so that there are no errors while
// creating a file/directory
fn sanitize(input: String) -> String {
    const AVOID: &str = "^~*+=`/\\\"><|";

    let mut out = input;
    out.retain(|c| !AVOID.contains(c));
    out = out.replace(':', "∶");
    out = out.replace('?', "﹖");
    out
}
