use crate::*;

/// Entry point
pub fn run() -> std::process::ExitCode {
    match run_with(cli::Cli::parse()) {
        Ok(_) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{} {err}", "error:".red());
            std::process::ExitCode::FAILURE
        }
    }
}

pub fn run_with(cli: cli::Cli) -> Result<(), Error> {
    match cli.subcommand {
        None => run_backup(&cli),
        Some(cli::CliCommand::List) => run_list(&cli),
        Some(cli::CliCommand::Wipe) => run_wipe(&cli),
        Some(cli::CliCommand::Diff { index }) => run_diff(&cli, index)
    }
}

