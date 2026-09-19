// SPDX-FileCopyrightText: 2026 Sayantan Santra <sayantan.santra689@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use clap_complete::shells::{Bash, Fish, Zsh};
use std::ffi::OsString;
use std::fs::{create_dir, remove_dir_all};

include!("src/cli.rs");

fn main() -> std::io::Result<()> {
    let autocomplete_target = "./target/autocomplete";
    remove_dir_all(autocomplete_target).ok();
    create_dir(autocomplete_target)?;
    let autocomplete_outdir = OsString::from(autocomplete_target);

    let mut cmd = get_command();
    clap_complete::generate_to(Bash, &mut cmd, "movie-rename", &autocomplete_outdir)?;
    clap_complete::generate_to(Fish, &mut cmd, "movie-rename", &autocomplete_outdir)?;
    clap_complete::generate_to(Zsh, &mut cmd, "movie-rename", &autocomplete_outdir)?;

    let man_target = "./target/man";
    remove_dir_all(man_target).ok();
    create_dir(man_target)?;
    let man_outdir = OsString::from(man_target);

    let man = clap_mangen::Man::new(cmd);
    man.generate_to(man_outdir)?;

    Ok(())
}
