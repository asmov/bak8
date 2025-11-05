use crate::*;
use crate::unixy::*;

pub fn cmd_which(name: &str) -> Command {
    let mut cmd = Command::new(CMD_WHICH);
    cmd.arg(name);
    cmd
}

/// returns an error if `which` doesn't exist or if the string returned is non-utf8
/// performs trim() against the string result, returning None if its empty afterwards
pub fn exec_which(name: &str) -> CrossResult<Option<Command>> {
    let cmd = cmd_which(name); 
    let which_path = CommandReturn::optional_utf8_from(CmdKind::Which, cmd)?
        .ok_or_else(|| CrossError::not_found(ErrNotFound::Which))?;
    
    Ok(Some(Command::new(which_path)))
}

/// calls `which {cmd_name}` and then runs that command with [open_file] as an argument
pub fn exec_which_env_editor() -> CrossResult<Option<Command>> {
    let editor_val = expand_env(ENV_VAR_EDITOR)?;
    let path = Path::new(&*editor_val);
    if path.is_absolute() && let Ok(path) = path.canonicalize() {
        Ok(Some(Command::new(path)))
    } else {
        exec_which(&*editor_val)
    }
}

pub const EDITOR_GUESSES: [&'static str; 5] = ["hx", "nvim", "vim", "vi", "nano"];

pub fn exec_any_which<S>(guesses: impl Iterator<Item = S>) -> Option<process::Command>
where
    S: AsRef<str>
{
    for guess in guesses {
        let Ok(Some(cmd)) = exec_which(guess.as_ref()) else { continue };
        return Some(cmd);
    }

    None
}

pub fn run_best_editor<P>(sys_open_cmd: Option<Command>, file: P, child_process: bool) -> CrossResult<CommandReturn>
where
    P: AsRef<Path> + Into<PathBuf>,
{
    let editor_cmd = if let Some(mut cmd) = sys_open_cmd {
        cmd.arg(file.as_ref());
        cmd
    } else if let Ok(Some(mut cmd)) = unixy::exec_which_env_editor() {
        cmd.arg(file.as_ref());
        cmd
    } else if let Some(mut cmd) = unixy::exec_any_which(unixy::EDITOR_GUESSES.iter()) {
        cmd.arg(file.as_ref());
        cmd
    } else {
        return CrossError::err_not_found(ErrNotFound::Editor)
    };

    CommandReturn::run(CmdKind::Editor, editor_cmd, child_process)
}

