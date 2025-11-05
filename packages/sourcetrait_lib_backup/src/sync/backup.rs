use std::path::PathBuf;
use crate::{paths::*, config::*, error::*, log::*, job::*, backup::*, cmd::rsync};
use super::Remote;

#[derive(Debug)]
pub struct SyncBackupJob {
    pub remote: Remote,
    pub remote_backup_storage_dir: PathBuf,
    pub backup_type: BackupType,
    pub backup_run_name: BackupRunName,
    pub source_dir: PathBuf,
    pub remote_incremental_source_dir: Option<PathBuf>,
    pub remote_dest_dir: SourceTraitBackupPath,
}

impl JobTrait for SyncBackupJob {
    type Output = SyncBackupJobOutput;

    fn run(&self, _config: &BackupConfig) -> Result<JobOutput> {
        log_info!("Began uploading {} backup of {} to remote {}",
            self.backup_type, self.backup_run_name.catalogue.tik_name(), self.remote.name.tik_name());

        self.verify_remote_environment()?;
        self.prepare_remote_dirs()?;

        let mut rsync_cmd = match self.backup_type {
            BackupType::Full => rsync::cmd_rsync_full_ssh(&self.source_dir, &self.remote, self.remote_dest_dir.as_path()),
            BackupType::Incremental => rsync::cmd_rsync_incremental_ssh(
                &self.source_dir,
                &self.remote.host,
                self.remote.user.as_ref().map(|s| s.as_str()),
                &self.remote_incremental_source_dir.as_ref().unwrap(),
                &self.remote_dest_dir.as_path()
            ),
        };

        let output = rsync_cmd.output().unwrap();

        if !output.status.success() {
            return Err(Error::rsync(output));
        }

        log_info!("Completed uploading {} backup of {} to {}",
            self.backup_type,
            self.backup_run_name.catalogue.tik_name(),
            format!("{}:{}", self.remote.name, self.remote_dest_dir.as_path().to_string_lossy()).tik_path());

        Ok(JobOutput::SyncBackup(SyncBackupJobOutput {
            remote: self.remote.clone(),
            source_dir: self.source_dir.clone(),
            backup_run_name: self.backup_run_name.clone(),
            remote_incremental_source_dir: self.remote_incremental_source_dir.clone(),
            remote_dest_dir: self.remote_dest_dir.clone()
        }))
    }
}

impl SyncBackupJob {
    fn verify_remote_environment(&self) -> Result<()> {
        let mut dirs: Vec<_> = backup_storage_subdirs(&self.remote_backup_storage_dir);
        dirs.insert(0, self.remote_backup_storage_dir.clone());

        let is_valid = crate::cmd::ssh_run::ssh_dirs_exist(&self.remote, &dirs)
            .map_err(|e| Error::Generic(format!("Failed to verify remote environment: {}", e)))?;

        match is_valid {
            true => Ok(()),
            false => Err(Error::Generic(format!("Some remote directories do not exist: {:?}", dirs)))
        }
    }

    fn prepare_remote_dirs(&self) -> Result<()> {
        let dirs = vec![self.remote_dest_dir.as_path()];
        let created = crate::cmd::ssh_run::ssh_make_dirs(&self.remote, &dirs)
            .map_err(|e| Error::Generic(format!("Failed to create remote directories: {}", e)))?;

        match created {
            true => Ok(()),
            false => Err(Error::Generic(format!("Failed to create remote directories: {:?}", dirs)))
        }
    }
}

#[derive(Debug)]
pub struct SyncBackupJobOutput {
    pub remote: Remote,
    pub source_dir: PathBuf,
    pub backup_run_name: BackupRunName,
    pub remote_incremental_source_dir: Option<PathBuf>,
    pub remote_dest_dir: SourceTraitBackupPath,
}

impl JobOutputTrait for SyncBackupJobOutput {}
