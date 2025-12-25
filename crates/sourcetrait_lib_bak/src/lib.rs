//! # SourceTrait Bak 
//!
//! Creates a backup `.bak` copy of **FILE**.
//!
//! Usage: `bak [OPTION]... FILE [DIR]`
//!
//! If **DIR** is not specified, the copy is created in the same directory as FILE.
//!
//! If *multiple* backups of FILE exist, the filename extension used will be: `.bak.N`.
//!
//! With multiple backups, the most recent backup will be always `bak.0`. Previous
//! copies will have their filename extension shifted by 1 (e.g., `bak.1` -> `bak.2`).
//!
//! Pruning (deletion) occurs after `-n NUM` backups.
//!
//! If the current backup is no *diff*erent than its predecessor, copying will be skipped.
//!
//! # Options
//!
//! - `-d`
//! Deletes all backup files for the source FILE.
//!
//! - `-n NUM`
//! Creates at most **NUM** backup files.
//! If not specified, defaults to 10 (0-9).

pub(crate) mod cli;
pub(crate) mod cli_run;
pub(crate) mod consts;
pub(crate) mod error;
pub(crate) mod os;
pub(crate) mod paths;
pub(crate) mod run;

pub use crate::{
    cli::{
        Cli, CliCommand,
    },
    consts::{
        SOURCETRAIT_BACKUP_SUBDIR,
    },
    run::{
        run, run_with,
    },
    paths::{
        mirror_dir,
    },
};

#[allow(unused_imports)]
pub(crate) use crate::{
    cli_run::*,
    consts::*,
    error::*,
    paths::*,
};

#[allow(unused_imports)]
pub(crate) use std::{
    fs, io::{self, Write}, path::{Path, PathBuf}
};
pub(crate) use clap::{Parser, Subcommand};
pub(crate) use colored::Colorize;
pub(crate) use sourcetrait_crossplat::{self as cross, prelude::*};
