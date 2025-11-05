use crate::{backup::*, cmd::xz, config::*, error::*, job::*, log::*, paths::*};
use std::path::PathBuf;

#[derive(Debug)]
pub struct ArchiveJob {
    pub(crate) backup_run_name: BackupRunName,
    pub(crate) source_dir: PathBuf,
    /// Bak8Path::Archive
    pub(crate) dest_filepath: Bak8Path,
}

#[derive(Debug)]
pub struct ArchiveJobOutput {
    pub backup_run_name: BackupRunName,
    pub source_dir: PathBuf,
    /// Bak8Path::Archive
    pub dest_filepath: Bak8Path,
    pub checksum: String,
}

impl JobTrait for ArchiveJob {
    type Output = ArchiveJobOutput;

    fn run(&self, config: &BackupConfig) -> Result<JobOutput> {
        log_info!(
            "Began archiving {}",
            self.backup_run_name.catalogue.tik_name()
        );

        self.dest_filepath.prepare(config)?;

        //todo: determine permissions for files created, based on the parent dir of dest_filepath

        let mut tar_xz_cmd = xz::cmd_tar_xz(&self.source_dir, self.dest_filepath.as_path());
        let output = tar_xz_cmd.output().unwrap();

        if !output.status.success() {
            return Err(Error::rsync(output));
        }

        let checksum = sha256::try_digest(self.dest_filepath.as_path())
            .map_err(|_| Error::Generic("Unable to checksum file".to_string()))?;

        let checksum_filepath = with_sha256_extension(self.dest_filepath.as_path());
        std::fs::write(
            &checksum_filepath,
            format!(
                "{checksum}  {filename}\n",
                filename = self
                    .dest_filepath
                    .as_path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
            ),
        )
        .map_err(|e| Error::file_io(e, &checksum_filepath, "Unable to write checksum file"))?;

        log_info!(
            "Completed archiving {} to {}",
            self.backup_run_name.catalogue.tik_name(),
            self.dest_filepath.tik_path()
        );

        Ok(JobOutput::Archive(ArchiveJobOutput {
            backup_run_name: self.backup_run_name.clone(),
            source_dir: self.source_dir.clone(),
            dest_filepath: self.dest_filepath.clone(),
            checksum: checksum,
        }))
    }
}

impl JobOutputTrait for ArchiveJobOutput {}

impl ArchiveJobOutput {
    pub fn new(
        backup_run_name: BackupRunName,
        source_dir: PathBuf,
        dest_filepath: Bak8Path,
        checksum: String,
    ) -> Self {
        Self {
            backup_run_name,
            source_dir,
            dest_filepath,
            checksum,
        }
    }
}
