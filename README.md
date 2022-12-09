# movie-rename

### A simple tool to rename movies, written in Rust.

This is made mostly due to [mnamer](https://github.com/jkwill87/mnamer) not having support for director's name, and partly because I wanted to try writing something useful in Rust.

## Installation
Install from [AUR](https://aur.archlinux.org/packages/movie_rename), my personal [lure-repo](https://github.com/SinTan1729/lure-repo) or download the binary from the releases.

## Notes

- The expected syntax is:

    `movie_rename <filename(s)> [-n|--dry-run] [-d|--directory] [-h|--help] [-v|--version]`
- There needs to be a config file named movie_rename.conf in your $XDG_CONFIG_HOME.
- It should consist of two lines. The first line should have your TMDb API key.
- The second line should have a pattern, that will be used for the rename.
- In the pattern, the variables need to be enclosed in {{}}, the supported variables are `title`, `year` and `director`.
- Default pattern is `{title} ({year}) - {director}`. Extension is always kept.
- Passing `--directory` assumes that the arguments are directory names, which contain exactly one movie and optionally subtitles.

- I plan to add more variables in the future. Support for TV Shows will not be added, since [tvnamer](https://github.com/dbr/tvnamer) does that excellently.