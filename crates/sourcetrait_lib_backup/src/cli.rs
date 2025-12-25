use crate::*;

#[derive(Parser, Debug)]
#[command(version, about = "Manages a rotational backup system")]
pub struct Cli {
    #[arg(short, help = "Configuration file [non-default]", value_parser = validate_config_file)]
    pub config_file: Option<PathBuf>,

    #[arg(short, help = "Force the operation without confirmation")]
    pub force: bool,

    #[arg(short, help = "Quit. Suppresses standard output")]
    pub quiet: bool,

    #[command(subcommand)]
    pub subcommand: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    #[command(subcommand, name = "backup", about = "Performs backups as configured")]
    Backup(CliBackupCommand),
    #[command(subcommand, name = "config", about = "Manages configuration")]
    Config(CliConfigCommand),
    #[command(subcommand, name = "log", about = "Reviews logs")]
    Log(CliLogCommand),
    #[command(name = "summary", alias="info", about = "Reviews a summary of recent backups")]
    Summary
}

#[derive(Subcommand, Debug)]
pub enum CliBackupCommand {
    #[command(name = "scheduled", alias = "cron", about = "Performs backups as scheduled")]
    Scheduled,
    #[command(name = "full", about = "Manually performs a full backup")]
    Full(ManualBackupCommand),
    #[command(name = "incremental", alias = "update", about = "Manually performs an incremental backup")]
    Incremental(ManualBackupCommand),
}

#[derive(Parser, Debug)]
pub struct ManualBackupCommand {
    #[arg(help = "Name of the backup to run")]
    pub name: String
}

#[derive(Subcommand, Debug)]
pub enum CliConfigCommand {
    #[command(name = "setup", about = "Initializes the user's backup configuration")]
    Setup,
    #[command(name = "edit", about = "Opens the backup configuration in their editor")]
    Edit,
    #[command(name = "verify", about = "Verifies the backup configuration")]
    Verify,
    #[command(name = "install", about = "Performs superuser setup of sourcetrait backup")]
    Install,
    #[command(name = "show", about = "Displays the backup configuration")]
    Show
}

#[derive(Subcommand, Debug)]
pub enum CliLogCommand {
    #[command(name = "list", about = "Lists backup log files")]
    List,
    #[command(name = "show", about = "Displays the log for a backup")]
    Show
}

fn validate_config_file(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path)
        .canonicalize()
        .map_err(|_| format!("Config file not found: {}", path.cyan()))?;

    if !path.is_file() {
        Err(format!("Config path is not a file: {}", path.to_str().unwrap().cyan()))
    } else {
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert()
    }
}
