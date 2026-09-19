use clap::{ArgAction, Command, ValueHint, arg, command};

// Bare command generation function to help with autocompletion
pub fn get_command() -> Command {
    command!()
        .name("movie-rename")
        .author("Sayantan Santra <sayantan.santra@gmail.com>")
        .about("A simple tool to rename movies, written in Rust.")
        .arg(arg!(-d --directory "Run in directory mode").action(ArgAction::SetTrue))
        .arg(arg!(-n --"dry-run" "Do a dry run").action(ArgAction::SetTrue))
        .arg(arg!(-l --"i-feel-lucky" "Always choose the first option").action(ArgAction::SetTrue))
        .arg(arg!(-i --"tmdb-id" <INTEGER> "Use a given TMDB ID. Does not work with directory mode."))
        .arg(
            arg!([entries] "The files/directories to be processed")
                .trailing_var_arg(true)
                .num_args(1..)
                .value_hint(ValueHint::AnyPath)
                .required(true),
        )
        // Use -v instead of -V for version
        .disable_version_flag(true)
        .arg(arg!(-v --version "Print version").action(ArgAction::Version))
        .arg_required_else_help(true)
        .help_template(
            "{before-help}{name} {version}\n{about}\nGPLv3 (c) {author}
                \n{usage-heading}\n{usage}\n\n{all-args}{after-help}",
        )
}
