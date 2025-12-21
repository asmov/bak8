pub mod backup;
pub mod config;
pub mod log;
pub mod summary;

use clap::Parser;
use crate::*;
use crate::{error::*, cli::*, config::*, log::*, run};

pub fn init(config: Option<&BackupConfig>, cli: Option<&Cli>) -> Result<()> {
    static DONE: OnceLock<()> = OnceLock::new();
    if DONE.get().is_none() {
        os_snapshot_init(OsSnapshotInit {})?;
        Log::init(config, cli);
        DONE.get_or_init(|| ());
    }
    
    Ok(())
}

pub fn run_main() -> ExitCode {
    let cli = Cli::parse();

    if let Ok(config) = read_cli_config(&cli) {
        match run_with_config(cli, config) {
            Ok(true) => ExitCode::SUCCESS,
            Ok(false) => ExitCode::FAILURE,
            Err(e) => {
                Log::get().error(&e.to_string());
                ExitCode::FAILURE
            }
        }
    } else {
        match run_with(cli) {
            Ok(true) => ExitCode::SUCCESS,
            Ok(false) => ExitCode::FAILURE,
            Err(e) => {
                Log::get().error(&e.to_string());
                ExitCode::FAILURE
            }
        }
    }
}

pub fn run_with(cli: Cli) -> Result<bool> {
    init(None, Some(&cli))?;
    match &cli.subcommand {
        Command::Backup(subcmd) => run::backup::run_backup(&cli, subcmd, None).map(|_| Ok(true))?,
        Command::Config(subcmd) => run::config::run_config(&cli, subcmd),
        Command::Log(subcmd) => run::log::run_log(&cli, subcmd),
        Command::Summary => run::summary::run_summary(&cli),
    }
}

pub fn run_with_config(cli: Cli, config: BackupConfig) -> Result<bool> {
    init(Some(&config), Some(&cli))?;
    match &cli.subcommand {
        Command::Backup(subcmd) => run::backup::run_backup(&cli, subcmd, Some(&config)).map(|_| Ok(true))?,
        Command::Config(subcmd) => run::config::run_config(&cli, subcmd),
        Command::Log(subcmd) => run::log::run_log(&cli, subcmd),
        Command::Summary => run::summary::run_summary(&cli),
    }
}