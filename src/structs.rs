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

        if let Some(year) = &self.year {
            buffer.push_str(&format!("({year}), "));
        }

        buffer.push_str(&format!(
            "Language: {}, ",
            get_long_lang(self.language.as_str())
        ));

        if let Some(director) = &self.director {
            buffer.push_str(&format!("Directed by: {director}, "));
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
    // List used from https://gist.github.com/carlopires/1262033/c52ef0f7ce4f58108619508308372edd8d0bd518#file-gistfile1-txt
    #[rustfmt::skip]
    static LANG_LIST: [(&str, &str); 185] = [("ab", "Abkhaz"), ("aa", "Afar"), ("af", "Afrikaans"), ("ak", "Akan"), ("sq", "Albanian"),
        ("am", "Amharic"), ("ar", "Arabic"), ("an", "Aragonese"), ("hy", "Armenian"), ("as", "Assamese"), ("av", "Avaric"),
        ("ae", "Avestan"), ("ay", "Aymara"), ("az", "Azerbaijani"), ("bm", "Bambara"), ("ba", "Bashkir"), ("eu", "Basque"),
        ("be", "Belarusian"), ("bn", "Bengali"), ("bh", "Bihari"), ("bi", "Bislama"), ("bs", "Bosnian"), ("br", "Breton"),
        ("bg", "Bulgarian"), ("my", "Burmese"), ("ca", "Catalan; Valencian"), ("ch", "Chamorro"), ("ce", "Chechen"),
        ("ny", "Chichewa; Chewa; Nyanja"), ("zh", "Chinese"), ("cv", "Chuvash"), ("kw", "Cornish"), ("co", "Corsican"),
        ("cr", "Cree"), ("hr", "Croatian"), ("cs", "Czech"), ("da", "Danish"), ("dv", "Divehi; Maldivian;"), ("nl", "Dutch"),
        ("dz", "Dzongkha"), ("en", "English"), ("eo", "Esperanto"), ("et", "Estonian"), ("ee", "Ewe"), ("fo", "Faroese"),
        ("fj", "Fijian"), ("fi", "Finnish"), ("fr", "French"), ("ff", "Fula"), ("gl", "Galician"), ("ka", "Georgian"),
        ("de", "German"), ("el", "Greek, Modern"), ("gn", "Guaraní"), ("gu", "Gujarati"), ("ht", "Haitian"), ("ha", "Hausa"),
        ("he", "Hebrew (modern)"), ("hz", "Herero"), ("hi", "Hindi"), ("ho", "Hiri Motu"), ("hu", "Hungarian"), ("ia", "Interlingua"),
        ("id", "Indonesian"), ("ie", "Interlingue"), ("ga", "Irish"), ("ig", "Igbo"), ("ik", "Inupiaq"), ("io", "Ido"), ("is", "Icelandic"),
        ("it", "Italian"), ("iu", "Inuktitut"), ("ja", "Japanese"), ("jv", "Javanese"), ("kl", "Kalaallisut"), ("kn", "Kannada"),
        ("kr", "Kanuri"), ("ks", "Kashmiri"), ("kk", "Kazakh"), ("km", "Khmer"), ("ki", "Kikuyu, Gikuyu"), ("rw", "Kinyarwanda"),
        ("ky", "Kirghiz, Kyrgyz"), ("kv", "Komi"), ("kg", "Kongo"), ("ko", "Korean"), ("ku", "Kurdish"), ("kj", "Kwanyama, Kuanyama"),
        ("la", "Latin"), ("lb", "Luxembourgish"), ("lg", "Luganda"), ("li", "Limburgish"), ("ln", "Lingala"), ("lo", "Lao"), ("lt", "Lithuanian"),
        ("lu", "Luba-Katanga"), ("lv", "Latvian"), ("gv", "Manx"), ("mk", "Macedonian"), ("mg", "Malagasy"), ("ms", "Malay"), ("ml", "Malayalam"),
        ("mt", "Maltese"), ("mi", "Māori"), ("mr", "Marathi (Marāṭhī)"), ("mh", "Marshallese"), ("mn", "Mongolian"), ("na", "Nauru"),
        ("nv", "Navajo, Navaho"), ("nb", "Norwegian Bokmål"), ("nd", "North Ndebele"), ("ne", "Nepali"), ("ng", "Ndonga"),
        ("nn", "Norwegian Nynorsk"), ("no", "Norwegian"), ("ii", "Nuosu"), ("nr", "South Ndebele"), ("oc", "Occitan"), ("oj", "Ojibwe, Ojibwa"),
        ("cu", "Old Church Slavonic"), ("om", "Oromo"), ("or", "Oriya"), ("os", "Ossetian, Ossetic"), ("pa", "Panjabi, Punjabi"), ("pi", "Pāli"),
        ("fa", "Persian"), ("pl", "Polish"), ("ps", "Pashto, Pushto"), ("pt", "Portuguese"), ("qu", "Quechua"), ("rm", "Romansh"), ("rn", "Kirundi"),
        ("ro", "Romanian, Moldavan"), ("ru", "Russian"), ("sa", "Sanskrit (Saṁskṛta)"), ("sc", "Sardinian"), ("sd", "Sindhi"), ("se", "Northern Sami"),
        ("sm", "Samoan"), ("sg", "Sango"), ("sr", "Serbian"), ("gd", "Scottish Gaelic"), ("sn", "Shona"), ("si", "Sinhala, Sinhalese"), ("sk", "Slovak"),
        ("sl", "Slovene"), ("so", "Somali"), ("st", "Southern Sotho"), ("es", "Spanish; Castilian"), ("su", "Sundanese"), ("sw", "Swahili"),
        ("ss", "Swati"), ("sv", "Swedish"), ("ta", "Tamil"), ("te", "Telugu"), ("tg", "Tajik"), ("th", "Thai"), ("ti", "Tigrinya"), ("bo", "Tibetan"),
        ("tk", "Turkmen"), ("tl", "Tagalog"), ("tn", "Tswana"), ("to", "Tonga"), ("tr", "Turkish"), ("ts", "Tsonga"), ("tt", "Tatar"), ("tw", "Twi"),
        ("ty", "Tahitian"), ("ug", "Uighur, Uyghur"), ("uk", "Ukrainian"), ("ur", "Urdu"), ("uz", "Uzbek"), ("ve", "Venda"), ("vi", "Vietnamese"),
        ("vo", "Volapük"), ("wa", "Walloon"), ("cy", "Welsh"), ("wo", "Wolof"), ("fy", "Western Frisian"), ("xh", "Xhosa"), ("yi", "Yiddish"),
        ("yo", "Yoruba"), ("za", "Zhuang, Chuang"), ("zu", "Zulu"), ("none", "None")];

    let long = LANG_LIST
        .iter()
        .filter(|x| x.0 == short)
        .map(|x| x.1)
        .next();

    if let Some(longlang) = long {
        String::from(longlang)
    } else {
        String::from(short)
    }
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
