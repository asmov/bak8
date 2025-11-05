use std::{env, path::Path, process};
use anyhow::anyhow;
use crate::{E_STR, CommandReturn};

use super::CrossPlatform;

const ENV_XDG_SESSION_TYPE: &str = "XDG_SESSION_TYPE";
const WHICH: &str = "which";

pub struct MacOsCrossPlatform;
impl CrossPlatform for MacOsCrossPlatform {
    fn in_terminal(&self) -> bool {
        env::var(ENV_XDG_SESSION_TYPE).is_err()
    }

    fn run_best_editor(&self, file: &Path, child_process: bool) -> anyhow::Result<CommandReturn> {
        let editor_cmd = if !self.in_terminal() {
            xdg_open_command(file)
        } else if let Some(cmd) = editor_command(file) {
            cmd
        } else if let Some(cmd) = guess_editor_command(file) {
            cmd
        } else {
            return Err(anyhow!("No editor found"))
        };

        CommandReturn::run(editor_cmd, child_process)
    }

    fn copy_file(&self, source: &Path, dest: &Path) -> std::io::Result<()> {
        macos_cp(source, dest)
            .or_else(|_| std::fs::copy(source, dest).map(|_| ()))
    }
    
    fn get_primary_user_group(&self) -> anyhow::Result<String> {
        let output = process::Command::new("id")
            .arg("-gn")
            .output()
            .map_err(|e| anyhow!("Failed to execute 'id -gn': {}", e))?;

        if !output.status.success() {
            return Err(anyhow!("'id -gn' command failed with status: {}", output.status));
        }

        let group_name = String::from_utf8(output.stdout)
            .map_err(|e| anyhow!("Failed to parse output of 'id -gn': {}", e))?
            .trim()
            .to_string();

        Ok(group_name)
    }
}

fn xdg_open_command(file: &Path) -> process::Command {
    let mut cmd = process::Command::new("xdg-open");
    cmd.arg(file);
    cmd
}

fn editor_command(file: &Path) -> Option<process::Command> {
    let which_editor = process::Command::new(WHICH)
        .arg("$EDITOR")
        .output()
        .ok()?;

    if !which_editor.status.success() || which_editor.stdout.is_empty() {
        return None;
    }

    let editor_path = String::from_utf8(which_editor.stdout).expect(E_STR);
    let mut cmd = process::Command::new(editor_path.trim());
    cmd.arg(file);
    Some(cmd)
}

const EDITOR_GUESSES: [&str; 4] = ["nvim", "vim", "vi", "nano"];

fn guess_editor_command(file: &Path) -> Option<process::Command> {
    for guess in EDITOR_GUESSES {
        let which_guess = process::Command::new(WHICH)
            .arg(guess)
            .output();

        let which_guess = match which_guess {
            Ok(which_guess) => which_guess,
            Err(_) => continue,
        };

        if !which_guess.status.success() || which_guess.stdout.is_empty() {
            continue;
        }

        let editor_path = String::from_utf8(which_guess.stdout).expect(E_STR);
        let mut cmd = process::Command::new(editor_path.trim());
        cmd.arg(file);
        return Some(cmd)
    }

    None
}

fn macos_cp(source: &Path, dest: &Path) -> std::io::Result<()> {
    let output = std::process::Command::new("cp")
        .arg("--preserve")
        .arg(source)
        .arg(dest)
        .output()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, String::from_utf8_lossy(&output.stderr).to_string()))
    }
}
