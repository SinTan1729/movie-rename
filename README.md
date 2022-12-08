# movie-rename

## A simple tool to reame movies, written in Rust.

This is made mostly due to [mnamer](https://github.com/jkwill87/mnamer) not having support for director's name, and partly because I wanted to try writing something useful in Rust.

The expected syntax is:

`movie_rename <filename(s)> [--dry-run]`
- There needs to be a config file names movie_rename.conf in your $XDG_CONFIG_HOME.
- It should consist of two lines. The first line should have your TMDb API key.
- The second line should have a pattern, that will be used for the rename.
- In the pattern, the variables need to be enclosed in {{}}, the supported variables are `title`, `year` and `director`.
- Default pattern is `{title} ({year}) - {director}`. Extension is always kept.

I plan to add more variables in the future. Support for TV Shows will not be added, since [tvnamer](https://github.com/dbr/tvnamer) does that excellently.