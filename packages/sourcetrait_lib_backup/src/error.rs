use crate::*;
use crate::log::*;

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("Config file error: {path} :: {cause}"))]
    ConfigFile { path: String, cause: String },

    #[snafu(display("Config parsing error :: {cause}"))]
    ConfigParse { cause: String },

    #[snafu(display("Config file {} not found. Have you ran {} yet?", path.tik_path(), "srctrait backup config".tik_cmd()))]
    DefaultConfigFileNotFound { path: String },

    #[snafu(display("Config file {} not found.", path.tik_path()))]
    ConfigFileNotFound { path: String },

    #[snafu(display("{message}: {path}{cause}", path = path.tik_path(),
        cause = cause.as_ref().map_or("".to_string(), |c| format!(" :: {c}"))))]
    FileIO{ message: String, path: String, cause: Option<String> },

    #[snafu(display("Config item {} not found for schema {}", name.tik_name(), schema.tik_name()))]
    ConfigReferenceNotFound { schema: &'static str, name: String },

    #[snafu(display("Directory {} not found. (config: {})", path.tik_path(), config_key.tik_name()))]
    ConfiguredDirNotFound { path: String, config_key: String },

    #[snafu(display("Subdirectory {} not found. (config: {})", path.tik_path(), config_key.tik_name()))]
    ConfiguredSubdirNotFound { path: String, config_key: String },

    #[snafu(display("Failed to {}: {cause}", "rsync".tik_cmd()))]
    Rsync { cause: String },

    #[snafu(display("Failed to {}: {cause}", "tar xz".tik_cmd()))]
    TarXZ { cause: String },

    #[snafu(display("Command {cmd} failed: {message}{cause}",
        cmd = cmd.to_string().tik_cmd(),
        cause = cause.as_ref().map_or("".to_string(), |c| format!(" :: {c}"))))]
    Cmd { cmd: crate::cmd::Cmd, message: String, cause: Option<String> },

    #[snafu(display("Remote command failed on host {remote_name}: {message}{cause}",
        remote_name = remote_name.tik_name(),
        cause = cause.as_ref().map_or("".to_string(), |c| format!(" :: {c}"))))]
    RemoteCmd { remote_name: String, message: String, cause: Option<String> },

    #[snafu(display("{account_type} account {account} not found",
        account = account.tik_name()))]
    AccountNotFound{ account_type: &'static str, account: String },
    
    Cross { source: cross::CrossError },

    #[snafu(display("{msg}"))]
    Generic { msg: String }
}

impl Error {
    pub const ACCOUNT_GROUP: &'static str = "Group";
    pub const ACCOUNT_USER: &'static str = "User";

    pub fn config_file(path: &Path, err: impl std::error::Error) -> Self {
        Self::ConfigFile {
            path: path.to_str().unwrap().to_string(),
            cause: err.to_string()
        }
    }

    pub fn configured_dir(path: &Path, config_key: &str, _e: std::io::Error) -> Self {
        Self::ConfiguredDirNotFound {
            config_key: config_key.to_string(),
            path: path.to_str().unwrap().to_string(),
        }
    }

    pub fn configured_subdir(path: &Path, config_key: &str, _e: std::io::Error) -> Self {
        Self::ConfiguredSubdirNotFound {
            config_key: config_key.to_string(),
            path: path.to_str().unwrap().to_string(),
        }
    }

    pub fn file_io(err: impl std::error::Error, path: &Path, message: &str) -> Self {
        Self::FileIO {
            message: message.to_string(),
            path: path.to_str().unwrap().to_string(),
            cause: Some(err.to_string())
        }
    }

    pub fn file_io_err(path: &Path, message: &str) -> Self {
        Self::FileIO {
            message: message.to_string(),
            path: path.to_str().unwrap().to_string(),
            cause: None
        }
    }

    const FAILED_TO_EXECUTE: &'static str = "Unable to run executable";

    pub fn cmd_execute(err: impl std::error::Error, cmd: crate::cmd::Cmd) -> Self {
        Self::Cmd {
            cmd: cmd,
            message: Self::FAILED_TO_EXECUTE.to_string(),
            cause: Some(err.to_string())
        }
    }

    pub fn cmd_output(output: std::process::Output, cmd: crate::cmd::Cmd, message: String) -> Self {
        Self::Cmd {
            cmd,
            message,
            cause: Some(String::from_utf8_lossy(&output.stderr).into_owned())
        }
    }

    pub fn remote_cmd(err: impl std::error::Error, remote: &crate::sync::Remote, message: &str) -> Self {
        Self::RemoteCmd {
            remote_name: remote.name.to_string(),
            message: message.to_string(),
            cause: Some(err.to_string())
        }
    }

    pub fn remote_cmd_err(remote: &crate::sync::Remote, message: &str) -> Self {
        Self::RemoteCmd {
            remote_name: remote.name.to_string(),
            message: message.to_string(),
            cause: None
        }
    }


    pub fn rsync(output: std::process::Output) -> Self {
        Self::Rsync {
            cause: String::from_utf8(output.stderr).unwrap()
        }
    }

    pub fn tar_xz(output: std::process::Output) -> Self {
        Self::TarXZ {
            cause: String::from_utf8(output.stderr).unwrap()
        }
    }
    
    pub fn msg<S: std::fmt::Display>(msg: S) -> Self {
        Self::Generic { msg: msg.to_string() }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<cross::CrossError> for Error {
    fn from(source: cross::CrossError) -> Self {
        Self::Cross { source }
    }
}
