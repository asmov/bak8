pub(crate) mod cli_run {
    pub(crate) mod backup;
    pub(crate) mod config;
    pub(crate) mod log;
    pub(crate) mod summary;
}
pub(crate) mod consts;
pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod log;
pub(crate) mod os_snapshot;
pub(crate) mod schedule;
pub(crate) mod paths;
pub(crate) mod cli;
pub(crate) mod cmd;
pub(crate) mod run;
pub(crate) mod backup;
pub(crate) mod job;
pub(crate) mod archive;
pub(crate) mod sync;
pub(crate) mod sys;

pub use crate::{
    archive::{
        ArchiveJob, ArchiveJobOutput,
    },
    consts::{
        ENV_SOURCETRAIT_BACKUP_HOME,
        BACKUP_ARCHIVE_DIRNAME,
        TAR_XZ_EXTENSION,
        SHA256_EXTENSION,
    },
    backup::{
        BackupType, BackupRunName, BackupJob, BackupJobOutput,
    },
    cli::{
        Cli, CliCommand, CliBackupCommand,
    },
    cli_run::{
        backup::run_backup,
    },
    cmd::{
        ssh_run::{
            ssh_dirs_exist,
            ssh_dir_manifest,
            ssh_delete_dir,
            ssh_files_exist,
            ssh_file_contents,
            ssh_temp_dir,
        },
    },
    config::{
        BackupConfig,
        BackupConfigArchive,
        BackupConfigBackup,
        BackupConfigRemote,
        BackupConfigRemoteGroup,
        BackupConfigSchedule,
        BackupConfigSync,
    },
    job::{
        JobOutput, JobResults,
    },
    paths::{
        SourceTraitBackupPath,
    },
    run::{
        init,
        run, run_with, run_with_config,
    },
    sync::{
        Remote,
        SyncPlatform,
        SyncArchiveJob, SyncArchiveJobOutput,
        SyncBackupJob, SyncBackupJobOutput,
    },
};

#[allow(unused_imports)]
pub(crate) use crate::{
    cli_run::{
        backup::*,
        config::*,
        log::*,
        summary::*,
    },
    archive::*,
    backup::*,
    cli::*,
    consts::*,
    cmd::*,
    config::*,
    error::*,
    job::*,
    log::*,
    os_snapshot::*,
    paths::*,
    schedule::*,
    sync::*,
    sys::*,
};

#[allow(unused_imports)]
pub(crate) use std::{
    borrow::Cow,
    env,
    fmt::{Display, Debug},
    fs,
    io::{self, Write},
    path::{PathBuf, Path},
    sync::{Arc, LazyLock, OnceLock},
    str::FromStr,
    process::{self, Command, ExitCode},
};

pub(crate) use clap::{Parser, Subcommand};
pub(crate) use chrono::{DateTime, Local, Timelike, TimeZone};
pub(crate) use colored::Colorize;
pub(crate) use sourcetrait_crossplat::{self as cross, prelude::*};
pub(crate) use sourcetrait_twostr::{self as twostr, *};
pub(crate) use validator::{Validate, ValidationError};
