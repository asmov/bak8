use crate::*;

fn init_log(config: Option<&BackupConfig>, cli: Option<&Cli>) -> BackupResult<()> {
    static DONE: OnceLock<()> = OnceLock::new();
    if DONE.get().is_none() {
        Log::init(config, cli);
        DONE.get_or_init(|| ());
    }
    
    Ok(())
}

fn init_os_snapshot() -> BackupResult<()> {
    static DONE: OnceLock<()> = OnceLock::new();
    if DONE.get().is_none() {
        os_snapshot_init(OsSnapshotInit {})?;
        DONE.get_or_init(|| ());
    }
    
    Ok(())
}

pub fn init(config: Option<&BackupConfig>, cli: Option<&Cli>) -> BackupResult<()> {
    init_os_snapshot()?;
    init_log(config, cli)
}

pub fn run() -> ExitCode {
    let cli = Cli::parse();
    match init_os_snapshot() {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error initializing OS snapshot");
            return ExitCode::FAILURE;
        },
    }

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

pub fn run_with(cli: Cli) -> BackupResult<bool> {
    init_os_snapshot()?;
    init_log(None, Some(&cli))?;
    match &cli.subcommand {
        CliCommand::Backup(subcmd) => run_backup(&cli, subcmd, None).map(|_| Ok(true))?,
        CliCommand::Config(subcmd) => run_config(&cli, subcmd),
        CliCommand::Log(subcmd) => run_log(&cli, subcmd),
        CliCommand::Summary => run_summary(&cli),
    }
}

pub fn run_with_config(cli: Cli, config: BackupConfig) -> BackupResult<bool> {
    init_os_snapshot()?;
    init_log(Some(&config), Some(&cli))?;
    match &cli.subcommand {
        CliCommand::Backup(subcmd) => run_backup(&cli, subcmd, Some(&config)).map(|_| Ok(true))?,
        CliCommand::Config(subcmd) => run_config(&cli, subcmd),
        CliCommand::Log(subcmd) => run_log(&cli, subcmd),
        CliCommand::Summary => run_summary(&cli),
    }
}