use clap_complete::generate_to;
use clap_complete::shells::{Bash, Fish, Zsh};
use std::env;
use std::ffi::OsString;
use std::fs::{create_dir, remove_dir_all};
use std::io::Error;

include!("src/args.rs");

fn main() -> Result<(), Error> {
    let target = "./target/autocomplete";
    remove_dir_all(target).ok();
    create_dir(target)?;
    let outdir = OsString::from(target);

    let mut cmd = get_command();
    generate_to(Bash, &mut cmd, "movie-rename", &outdir)?;
    generate_to(Fish, &mut cmd, "movie-rename", &outdir)?;
    generate_to(Zsh, &mut cmd, "movie-rename", &outdir)?;
    Ok(())
}
