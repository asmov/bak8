use std::{fs, io::Write, path::Path};
use colored::Colorize;
use crate::{error::*, log::*, config::*, cli::*, paths::*};
use crate::*;

pub(crate) fn run_config(cli: &Cli, subcmd: &ConfigCommand) -> Result<bool> {
    let config_path = select_config_path(&cli)?;
    match subcmd {
        ConfigCommand::Setup => run_config_setup(&config_path, cli.force),
        ConfigCommand::Edit => run_config_edit(&config_path),
        ConfigCommand::Verify => run_config_verify(&config_path),
        ConfigCommand::Install => run_config_install(&config_path),
        ConfigCommand::Show => run_config_show(&config_path),
    }
}

fn run_config_setup(config_path: &Path, force: bool) -> Result<bool> {
    if config_path.exists() {
        println!("Verifying config file: {}", config_path.tikn_path());
        return run_config_verify(config_path);
    }

    if !force {
        println!("{} Config file not found: {}",
            "warning:".tikn_warning(), config_path.tikn_path());
        if !confirm("Would you like to create it now?")? {
            return Ok(false);
        }
    }

    let config_dir = config_path.parent()
        .ok_or_else(|| Error::file_io_err(config_path, "Unable to determine config file parent directory"))?;
    fs::create_dir_all(config_dir)
        .map_err(|e| Error::file_io(e, config_dir, "Failed to create config path directories"))?;
    fs::write(config_path, CONFIG_DEFAULTS)
        .map_err(|e| Error::file_io(e, config_path, "Failed to write default config"))?;

    println!("Config file created: {}", config_path.tikn_path());
    println!("Edit your config with {}\nValidate your config with {}",
        "bak8 config edit".tikn_cmd(), "bak8 config verify".tikn_cmd());

    if confirm("Would you like to edit it now?")? {
        run_config_edit(config_path)
    } else {
        Ok(true)
    }
}

fn confirm(question: &str) -> Result<bool> {
    print!("{} {question} {} ", "confirm:".tikn_confirm(), "[y/N]:".tikn_prompt());

    std::io::stdout().flush()
        .expect("Failed to flush stdout");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => Ok(true),
        _ => Ok(false),
    }
}

fn run_config_edit(config_path: &Path) -> Result<bool> {
    eprintln!("Launching editor for config file: {}", config_path.tikn_path());

    let output = PLATFORM.run_best_editor(config_path, false)
        .map_err(|e| Error::Generic(format!("Failed to run editor :: {e}")))?
        .unwrap_output();

    if !output.status.success() {
        return Err(Error::Generic(format!("Failed to run editor :: {}", String::from_utf8(output.stderr).unwrap())));
    }

    println!("Verifying edit");
    run_config_verify(config_path)
}

/// Checks whether config_path exists and prints an error message if it does not.
/// Returns true if an error message was printed, false otherwise.
fn handle_config_file_not_found(config_path: &Path) -> bool {
    if !config_path.exists() {
        eprintln!("{} Config file not found: {}\n       Run {} to create it.",
            "error:".tikn_error(),
            config_path.to_str().unwrap().tikn_path(),
            "bak8 config setup".tikn_cmd());
        true
    } else {
        false
    }
}

/// Returns the config file if it exists and is valid, otherwise returns None if the error was handled.
fn verify_config_file(config_path: &Path) -> Result<Option<BackupConfig>> {
    if handle_config_file_not_found(config_path) {
        return Ok(None);
    }

    let config = match read_config(Some(config_path)) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{} Config is invalid :: {e}", "error:".tikn_error());
            return Ok(None);
        }
    };

    Ok(Some(config))
}

fn needs_install(config: &BackupConfig) -> bool {
    verify_backup_dirs(config).is_err()
}

/// Ran with superuser privileges to install bak8. (sudo)
// Keep operations to a minimum to avoid security risks.
fn run_config_install(config_path: &Path) -> Result<bool> {
    let config = match verify_config_file(config_path)? {
        Some(config) => config,
        None => return Ok(false), // should we tell them to run `bak8 config setup` again?
    };

    if needs_install(&config) {
        let storage_dir = Bak8Path::StorageDir(config.backup_storage_dir_path());
        storage_dir.setup(&config)?;
    }

    verify_backup_dirs(&config)?;

    println!("Installation complete.");
    Ok(true)
}

fn run_config_verify(config_path: &Path) -> Result<bool> {
    println!("Config is valid.");

    let config = match verify_config_file(config_path)? {
        Some(config) => config,
        None => return Ok(false),
    };

    if needs_install(&config) {
        eprintln!("Storage directories need to be installed.");
        eprintln!("Run {} to install storage directories.", "sudo bak8 config install".tikn_cmd());
        Ok(false)
    } else {
        Ok(true)
    }
}

fn run_config_show(config_path: &Path) -> Result<bool> {
    if handle_config_file_not_found(config_path) {
        return Ok(false);
    }

    let header = format!("sourcetrait backup config: {}", config_path.tikn_path());
    println!("{}", header.cyan());
    println!("{:=<1$}", "".cyan(), header.chars().count());
    print!("{}", fs::read_to_string(config_path)
        .map_err(|e| Error::Generic(format!("Unable to read from config file: {} :: {e}", config_path.tikn_path())))?);

    Ok(true)
}
