![latest-release](https://img.shields.io/github/v/release/SinTan1729/movie-rename?label=latest%20release) ![commits-since-latest-release](https://img.shields.io/github/commits-since/SInTan1729/movie-rename/latest?label=commits%20since%20latest%20release)
# movie-rename

### A simple tool to rename movies, written in Rust.

This is made mostly due to [mnamer](https://github.com/jkwill87/mnamer) not having support for director's name, and partly because I wanted to try writing something useful in Rust.

## Installation
Install from [AUR](https://aur.archlinux.org/packages/movie-rename), my personal [lure-repo](https://github.com/SinTan1729/lure-repo) or download the binary from the releases.

## Notes

- The expected syntax is:

    `movie-rename <filename(s)> [-n|--dry-run] [-d|--directory] [-h|--help] [-v|--version]`
- There needs to be a config file named `config` in the `$XDG_CONFIG_HOME/movie-rename/` directory.
- It should consist of two lines. The first line should have your TMDb API key.
- The second line should have a pattern, that will be used for the rename.
- In the pattern, the variables need to be enclosed in {{}}, the supported variables are `title`, `year` and `director`.
- Default pattern is `{title} ({year}) - {director}`. Extension is always kept.
- Passing `--directory` or `-d` assumes that the arguments are directory names, which contain exactly one movie and optionally subtitles.
- Passing `--dry-run` or `-n` does a dry tun and only prints out the new names, without actually doing anything.
- Passing `-nd` or `-dn` does a dry run in directory mode.
- Passing `--help` or `-h` shows help and exits.
- Passing `--version` or `-v` shows version and exits.

- I plan to add more variables in the future. Support for TV Shows will not be added, since [tvnamer](https://github.com/dbr/tvnamer) does that excellently.