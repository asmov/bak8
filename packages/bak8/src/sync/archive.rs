use std::path::PathBuf;
use crate::{backup::*, config::*, cmd::ssh_run, error::*, job::*, log::*, paths::*};
use super::Remote;

#[derive(Debug)]
pub struct SyncArchiveJob {
    pub(crate) backup_run_name: BackupRunName,
    pub(crate) remote: Remote,
    pub(crate) source_filepath: PathBuf,
    pub(crate) remote_dest_filepath: PathBuf
}

impl JobTrait for SyncArchiveJob {
    type Output = SyncArchiveJobOutput;

    fn run(&self, _config: &BackupConfig) -> Result<JobOutput> {
        log_info!("Began uploading archive of {} to remote {}",
            self.backup_run_name.catalogue.tik_name(), self.remote.name.tik_name());
        
        self.prepare_remote_dirs()?;

        let source_checksum_filepath = with_sha256_extension(&self.source_filepath);
        let dest_checksum_filepath = with_sha256_extension(&self.remote_dest_filepath);
        
        ssh_run::scp_file(&self.remote, &self.source_filepath, &self.remote_dest_filepath)?;
        ssh_run::scp_file(&self.remote, &source_checksum_filepath, &dest_checksum_filepath)?;
        ssh_run::ssh_check_file_checksum(&self.remote, &self.remote_dest_filepath)?;

        log_info!("Completed uploading archive of {} to remote {}",
            self.backup_run_name.catalogue.tik_name(), self.remote.name.tik_name());

        Ok(JobOutput::SyncArchive(SyncArchiveJobOutput {
            backup_run_name: self.backup_run_name.clone(),
            remote: self.remote.clone(),
            source_filepath: self.source_filepath.clone(),
            remote_dest_filepath: self.remote_dest_filepath.clone(),
            remote_dest_checksum_filepath: dest_checksum_filepath 
        }))
    }
}

impl SyncArchiveJob {
    fn prepare_remote_dirs(&self) -> Result<()> {
        let dir = self.remote_dest_filepath.parent()
            .ok_or_else(|| Error::file_io_err(&self.remote_dest_filepath, "Failed to get parent directory"))?;

        let created = crate::cmd::ssh_run::ssh_make_dirs(&self.remote, &vec![dir])
            .map_err(|e| Error::Generic(format!("Failed to create remote directories: {}", e)))?;

        match created {
            true => Ok(()),
            false => Err(Error::Generic(format!("Failed to create remote directory: {}", dir.to_str().unwrap())))
        }
    }
}


#[derive(Debug)]
pub struct SyncArchiveJobOutput {
    pub backup_run_name: BackupRunName,
    pub remote: Remote,
    pub source_filepath: PathBuf,
    pub remote_dest_filepath: PathBuf,
    pub remote_dest_checksum_filepath: PathBuf
}

impl JobOutputTrait for SyncArchiveJobOutput {}


