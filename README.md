<!-- SPDX-FileCopyrightText: 2026 Sayantan Santra <sayantan.santra689@gmail.com> -->
<!-- SPDX-License-Identifier: GPL-3.0-only -->

[![latest-release](https://img.shields.io/github/v/release/SinTan1729/movie-rename?label=latest%20release)](https://github.com/SinTan1729/movie-rename/releases/latest/)
![commits-since-latest-release](https://img.shields.io/github/commits-since/SInTan1729/movie-rename/latest?label=commits%20since%20latest%20release)
[![AUR package](https://img.shields.io/aur/version/movie-rename-bin?label=AUR&logo=archlinux)](https://aur.archlinux.org/packages/movie-rename-bin/)

# `movie-rename`

### A simple tool to rename movies, written in Rust.

It turns a file like `Apur.Sansar.HEVC.2160p.AC3.mkv` into `Apur Sansar (1959) - Satyajit Ray.mkv` using metadata pulled from [TMDB](https://www.themoviedb.org/).

This is made mostly due to [mnamer](https://github.com/jkwill87/mnamer) not having support for director's name, and also because I wanted to try writing something useful in Rust.

## Installation

Install from [AUR](https://aur.archlinux.org/packages/movie-rename-bin), my personal [lure-repo](https://github.com/SinTan1729/lure-repo) or download the binary from the releases. You can also get it from [crates.io](https://crates.io/crates/movie-rename).

You can also install from source by using

```
git clone https://github.com/SinTan1729/movie-rename
cd movie-rename
sudo make install
```

## Usage

```
movie-rename [OPTIONS] <entries>...

Arguments:
  <entries>...  The files/directories to be processed

Options:
  -d, --directory          Run in directory mode
  -n, --dry-run            Do a dry run
  -l, --i-feel-lucky       Always choose the first option
  -i, --tmdb-id <INTEGER>  Use a given TMDB ID. Does not work with directory mode.
  -v, --version            Print version
  -h, --help               Print help
```

## Notes

- Currently, it only supports names in English. It should be easy to turn it into a configurable option. Since for movies in all the languages I know, English name is usually provided, it's a non-feature for me. If someone is willing to test it out for other languages, they're welcome. I'm open to accepting PRs.
- I plan to add more variables in the future. Support for TV Shows will not be added, since [tvnamer](https://github.com/dbr/tvnamer) does that excellently.
