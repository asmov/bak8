use std::{path::{Path, PathBuf}, process};
use crate::{error::*, log::TikPath, sync::*};

const SSH_CMD: &str = "ssh";
const SCP_CMD: &str = "scp";
const E_SSH_CMD: &str = "Failed to run ssh";
const E_SCP_CMD: &str = "Failed to run scp";

pub fn ssh_dirs_exist<P: AsRef<Path>>(remote: &Remote, dirs: &Vec<P>) -> Result<bool> {
    ssh_test_paths(remote, 'd', dirs)
}

pub fn ssh_files_exist<P: AsRef<Path>>(remote: &Remote, dirs: &Vec<P>) -> Result<bool> {
    ssh_test_paths(remote, 'f', dirs)
}

pub fn ssh_test_paths<P: AsRef<Path>>(remote: &Remote, test_flag: char, paths: &Vec<P>) -> Result<bool> {
    let mut ssh_cmd = process::Command::new(SSH_CMD);
    ssh_cmd.arg(remote.addr());

    let cmd = paths.into_iter()
        .map(|dir| format!("test -{test_flag} '{dir}'",
            dir = dir.as_ref().to_str().unwrap()))
        .collect::<Vec<_>>()
        .join(" && ");

    ssh_cmd.arg(cmd);

    let output = ssh_cmd.output()
        .map_err(|e| Error::remote_cmd(e, remote, E_SSH_CMD))?;

    Ok(output.status.success())
}


pub fn ssh_make_dirs<P: AsRef<Path>>(remote: &Remote, dirs: &Vec<P>) -> Result<bool> {
    let mut ssh_cmd = process::Command::new(SSH_CMD);
    ssh_cmd.arg(remote.addr());

    let cmd = dirs.into_iter()
        .map(|dir| format!("mkdir -p '{dir}'",
            dir = dir.as_ref().to_str().unwrap()))
        .collect::<Vec<_>>()
        .join(" && ");

    ssh_cmd.arg(cmd);

    let output = ssh_cmd.output()
        .map_err(|e| Error::remote_cmd(e, remote, E_SSH_CMD))?;

    Ok(output.status.success())
}

/// Returns contents if the file exists, otherwise None
pub fn ssh_file_contents<P: AsRef<Path>>(remote: &Remote, file: P) -> Result<Option<String>> {
    let output = process::Command::new(SSH_CMD)
        .arg(remote.addr())
        .arg(format!("cat '{file}' 2>/dev/null",
            file = file.as_ref().to_str().unwrap()))
        .output()
        .map_err(|e| Error::remote_cmd(e, remote, E_SSH_CMD))?;

    match output.status.success() {
        true => Ok(Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())),
        false => Ok(None)
    }
}

/// Verifies that SHA256 checksum matches the file if: it exists and has a .sha256 file with it
pub fn ssh_check_file_checksum<P: AsRef<Path>>(remote: &Remote, file: P) -> Result<()> {
    let parent_dir = file.as_ref().parent()
        .ok_or_else(|| Error::msg("File has no parent directory".to_string()))?;

    let output = process::Command::new(SSH_CMD)
        .arg(remote.addr())
        .arg(format!("cd {parent_dir} && sha256sum --quiet -c '{file}.sha256'",
            parent_dir = parent_dir.to_str().unwrap(),
            file = file.as_ref().to_str().unwrap()))
        .output()
        .map_err(|e| Error::remote_cmd(e, remote, E_SSH_CMD))?;

    match output.status.success() {
        true => Ok(()),
        false => Err(Error::remote_cmd_err(remote, &format!(
            "Failed to verify remote file checksum: {} :: {}",
                file.as_ref().tik_path(),
                String::from_utf8_lossy(&output.stderr))))
    }
}


/// Returns contents if the file exists, otherwise None
pub fn ssh_temp_dir(remote: &Remote) -> Result<PathBuf> {
    let output = process::Command::new(SSH_CMD)
        .arg(remote.addr())
        .arg("mktemp -d")
        .output()
        .map_err(|e| Error::remote_cmd(e, remote, E_SSH_CMD))?;

    match output.status.success() {
        true => Ok(PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())),
        false => Err(Error::remote_cmd_err(remote, &format!(
            "Unable to create temp dir :: {}", String::from_utf8_lossy(&output.stderr)))) 
    }
}

/// Writes contents to a remote file
pub fn ssh_write_file<P: AsRef<Path>>(remote: &Remote, file: P, contents: &str) -> Result<()> {
    let contents = contents.replace("\'", "\\\'"); // escape '
    let output = process::Command::new(SSH_CMD)
        .arg(remote.addr())
        .arg(format!("echo '{contents}' > {file} 2>/dev/null",
            file = file.as_ref().to_str().unwrap()))
        .output()
        .map_err(|e| Error::remote_cmd(e, remote, E_SSH_CMD))?;

    match output.status.success() {
        true => Ok(()),
        false => Err(Error::remote_cmd_err(remote, &format!("Failed to write to remote file {} :: {}",
            file.as_ref().tik_path(), String::from_utf8_lossy(&output.stderr))))
    }
}

/// Deletes a remote directory tree
pub fn ssh_delete_dir(remote: &Remote, dir: &Path) -> Result<PathBuf> {
    let output = process::Command::new(SSH_CMD)
        .arg(remote.addr())
        .arg(format!("rm -rf '{dir}'",
            dir = dir.to_str().unwrap()))
        .output()
        .map_err(|e| Error::remote_cmd(e, remote, E_SSH_CMD))?;

    match output.status.success() {
        true => Ok(PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())),
        false => Err(Error::remote_cmd_err(remote, &format!(
            "Unable to create temp dir :: {}", String::from_utf8_lossy(&output.stderr)))) 
    }
}

/// Creates a manifest for a remote directory, including sha256 checksums for files.
// These commands should match up with the `tools/update-testing-manifests.bash` script.
pub fn ssh_dir_manifest(remote: &Remote, dir: &Path) -> Result<String> {
    let output = process::Command::new(SSH_CMD)
        .arg(remote.addr())
        .arg(format!("cd '{dir}' ; find . -type d | sort ; find . -type f -exec sha256sum {{}} \\; | sort -k 2",
            dir = dir.to_str().unwrap()))
        .output()
        .map_err(|e| Error::remote_cmd(e, remote, E_SSH_CMD))?;

    match output.status.success() {
        true => Ok(String::from_utf8(output.stdout).unwrap().trim().to_owned()),
        false => Err(Error::remote_cmd_err(remote, &format!(
            "Unable to dump a directory manifest :: {}", String::from_utf8_lossy(&output.stderr)))) 
    }
}

pub fn scp_file(remote: &Remote, local_file: &Path, remote_dest: &Path) -> Result<()> {
    let output = process::Command::new(SCP_CMD)
        .arg(local_file)
        .arg(format!("{remote}:{dest}",
            remote = remote.addr(),
            dest = remote_dest.to_str().unwrap()))
        .output()
        .map_err(|e| Error::remote_cmd(e, remote, E_SCP_CMD))?;

    match output.status.success() {
        true => Ok(()),
        false => Err(Error::remote_cmd_err(remote, &format!(
            "Unable to remote copy file :: {}", String::from_utf8_lossy(&output.stderr)))) 
    }
}


