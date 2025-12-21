use semver;
use crate::*;
use crate::{backup::{BackupRunName, BackupType}, config, error::*, log::*, sys::*, config::*};

//const BACKUP_STORAGE_DIR_MODE: u32 = 0o750; //todo remove
const BACKUP_STORAGE_DIR_PERMISSIONS: cross::BasicPermissionMode = cross::BasicPermissionMode {
    user: cross::BasicPermissionSet { read: true, write: true, execute: true },
    group: cross::BasicPermissionSet { read: true, write: false, execute: true },
    ..cross::BasicPermissionMode::DEFAULT
};

const BACKUP_STORAGE_FILE_PERMISSIONS: cross::BasicPermissionMode = cross::BasicPermissionMode {
    user: cross::BasicPermissionSet { read: true, write: true, execute: false },
    group: cross::BasicPermissionSet { read: true, write: false, execute: false },
    ..cross::BasicPermissionMode::DEFAULT
};

//const BACKUP_RUN_DIR_UNIX_MODE: u32 = 0o700; //todo remove
const BACKUP_RUN_DIR_PERMISSIONS: cross::BasicPermissionMode = cross::BasicPermissionMode {
    user: cross::BasicPermissionSet {
        read: true,
        write: true,
        execute: true,
    },
    ..cross::BasicPermissionMode::DEFAULT
};

pub mod consts {
    pub const HOME_CONFIG_DIR: &'static str = ".config/sourcetrait/backup";
    pub const SOURCETRAIT_BACKUP_CONFIG_FILENAME: &'static str = "backup.toml";
    pub const BACKUP_ARCHIVE_DIRNAME: &'static str = "archive";
    pub const BACKUP_FULL_DIRNAME: &'static str = "full";
    pub const BACKUP_INCREMENTAL_DIRNAME: &'static str = "incremental";
    pub const BACKUP_LOGS_DIRNAME: &'static str = "logs";
    pub const SOURCETRAIT_BACKUP_FS_VERSION_FILENAME: &'static str = ".backup_fs_version";
    pub const ENV_SOURCETRAIT_BACKUP_HOME: &'static str = "BACKUP_HOME";
    pub const TAR_XZ_EXTENSION: &'static str = "tar.xz";
    pub const SHA256_EXTENSION: &str = "sha256";
}

#[derive(Debug, Clone)]
pub struct BackupPathParts {
    pub backup_type: BackupType,
    pub hostname: String,
    pub username: String,
    pub catalogue: String,
    pub datetime: Option<DateTime<Local>>,
}

impl BackupPathParts {
    pub fn new(backup_type: BackupType, hostname: &str, username: &str, catalogue: &str, datetime: Option<&DateTime<Local>>) -> Self {
        Self {
            backup_type,
            hostname: hostname.to_string(),
            username: username.to_string(),
            datetime: datetime.and_then(|dt| Some(dt.clone())),
            catalogue: catalogue.to_string()
        }
    }

    pub fn from_run(backup_type: BackupType, run_name: &BackupRunName) -> Self {
        Self {
            backup_type,
            hostname: run_name.hostname.clone(),
            username: run_name.username.clone(),
            catalogue: run_name.catalogue.clone(),
            datetime: Some(run_name.datetime.clone()),
        }
    }
}

impl BackupType {
    pub fn subdir_name(&self) -> &'static str {
        match self {
            BackupType::Full => consts::BACKUP_FULL_DIRNAME,
            BackupType::Incremental => consts::BACKUP_INCREMENTAL_DIRNAME,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SourceTraitBackupPath {
    StorageDir(PathBuf),
    BackupDir{ storage_dir: PathBuf, path_parts: BackupPathParts, path: PathBuf },
    FullBackup{ storage_dir: PathBuf, run_name: BackupRunName, path: PathBuf},
    IncrementalBackup{ storage_dir: PathBuf, run_name: BackupRunName, path: PathBuf },
    Archive{ storage_dir: PathBuf, run_name: BackupRunName, path: PathBuf },
    Log{ storage_dir: PathBuf, run_name: BackupRunName, path: PathBuf },
    UserConfig{ home_dir: PathBuf, path: PathBuf},
    FileSytemVersion { storage_dir: PathBuf, path: PathBuf },
    RemoteStorageDir{ remote: crate::sync::Remote, storage_dir: PathBuf },
}

impl AsRef<Path> for SourceTraitBackupPath {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl TikPath for SourceTraitBackupPath {
    fn tik_path(&self) -> String {
        self.as_path().tik_path()
    }

    fn tikn_path(&self) -> String {
        self.as_path().tikn_path()
    }
}

impl SourceTraitBackupPath {
    pub fn storage_dir<P: AsRef<Path>>(storage_dir: P) -> Self {
        Self::StorageDir(storage_dir.as_ref().to_path_buf())
    }

    pub fn full_backup<P: AsRef<Path>>(storage_dir: P, run_name: &BackupRunName) -> Self {
        Self::FullBackup {
            storage_dir: storage_dir.as_ref().to_path_buf(),
            path: storage_dir.as_ref()
                .join(consts::BACKUP_FULL_DIRNAME)
                .join(&run_name.username)
                .join(&run_name.hostname)
                .join(run_name.datetime.format("%Y").to_string())
                .join(run_name.datetime.format("%m").to_string())
                .join(run_name.datetime.format("%d").to_string())
                .join(run_name),
            run_name: run_name.clone()
        }
    }

    pub fn incremental_backup<P: AsRef<Path>>(storage_dir: P, run_name: &BackupRunName) -> Self {
        Self::IncrementalBackup {
            storage_dir: storage_dir.as_ref().to_path_buf(),
            path: storage_dir.as_ref()
                .join(consts::BACKUP_INCREMENTAL_DIRNAME)
                .join(&run_name.username)
                .join(&run_name.hostname)
                .join(run_name.datetime.format("%Y").to_string())
                .join(run_name.datetime.format("%m").to_string())
                .join(run_name.datetime.format("%d").to_string())
                .join(run_name),
            run_name: run_name.clone()
        }
    }

    pub fn backup<P: AsRef<Path>>(storage_dir: P, backup_type: BackupType, run_name: &BackupRunName) -> Self {
        match backup_type {
            BackupType::Full => Self::full_backup(storage_dir, run_name),
            BackupType::Incremental => Self::incremental_backup(storage_dir, run_name),
        }
    }

    pub fn backup_dir<P: AsRef<Path>>(storage_dir: P, path_parts: BackupPathParts) -> Self {
        let mut path = storage_dir.as_ref()
            .join(&path_parts.backup_type.subdir_name())
            .join(&path_parts.username)
            .join(&path_parts.hostname);

        if let Some(datetime) = path_parts.datetime {
            path.push(datetime.format("%Y").to_string());
            path.push(datetime.format("%m").to_string());
            path.push(datetime.format("%d").to_string());
        }

        Self::BackupDir {
            storage_dir: storage_dir.as_ref().to_path_buf(),
            path,
            path_parts,
        }
    }

    /// Creates a path for an archive file.
    pub fn archive<P: AsRef<Path>>(storage_dir: P, run_name: &BackupRunName) -> Self {
        Self::Archive {
            storage_dir: storage_dir.as_ref().to_path_buf(),
            path: storage_dir.as_ref().join(consts::BACKUP_ARCHIVE_DIRNAME)
                .join(&run_name.username)
                .join(&run_name.hostname)
                .join(run_name.datetime.format("%Y").to_string())
                .join(run_name.datetime.format("%m").to_string())
                .join(run_name.datetime.format("%d").to_string())
                .join(&run_name)
                .with_extension(consts::TAR_XZ_EXTENSION),
            run_name: run_name.clone()
        }
    }

    pub fn log<P: AsRef<Path>>(storage_dir: P, run_name: &BackupRunName) -> Self {
        Self::Log {
            storage_dir: storage_dir.as_ref().to_path_buf(),
            path: storage_dir.as_ref().join(consts::BACKUP_LOGS_DIRNAME).join(&run_name),
            run_name: run_name.clone()
        }
    }

    pub fn user_config<P: AsRef<Path>>(home_dir: P) -> Self {
        Self::UserConfig {
            home_dir: home_dir.as_ref().to_path_buf(),
            path: home_dir.as_ref().join(consts::HOME_CONFIG_DIR).join(consts::SOURCETRAIT_BACKUP_CONFIG_FILENAME)
        }
    }

    pub fn fs_version<P: AsRef<Path>>(storage_dir: P) -> Self {
        Self::FileSytemVersion {
            storage_dir: storage_dir.as_ref().to_path_buf(),
            path: storage_dir.as_ref().join(consts::SOURCETRAIT_BACKUP_FS_VERSION_FILENAME),
        }
    }

    /// Reads the file system version file IF self is a [Bak8Path::FileSytemVersion].
    pub fn read_fs_version(&self) -> Result<semver::Version> {
        assert!(matches!(self, SourceTraitBackupPath::FileSytemVersion{..}), "Invalid path type for reading file system version");

        fs::read_to_string(self.as_path())
            .map_err(|e| Error::file_io(e, self.as_path(), "Failed to read file system version file"))
            .map(|version_str| semver::Version::parse(&version_str))?
            .map_err(|e| Error::file_io(e, self.as_path(), "Failed to parse file system version file"))
    }
    
    /// Performs one-time setup of a path if necessary, based on global configuration values such as `backup_storage_dir`.
    /// This includes creating directories and setting permissions.
    /// Currently, this only applies to [Bak8Path::StorageDir].
    pub fn setup(&self, config: &BackupConfig) -> Result<()> {
        let current_user = os_snapshot().current_user();
        let current_user_group_aid_capable = os_snapshot().current_user_primary_group_aid();
        let admin_uaid = storage_admin_aid(config)?;
        let backup_users_gaid = backup_users_id(config)?;
        let backup_users_gaid_capable = cross::Capable::Capable(backup_users_gaid);
        let backup_users_group = backup_users_group(config)?;
        
        match self {
            Self::StorageDir(path) => {
                if !path.exists() {
                    fs::create_dir_all(path)
                        .map_err(|e| Error::file_io(e, path, "Failed to create backup storage directory"))?;
                    cross::PLATFORM.fs()
                        .own_capable(&path, admin_uaid, &backup_users_gaid_capable, BACKUP_STORAGE_DIR_PERMISSIONS)
                        .map_err(|e| Error::file_io(e, &path, "Failed to set permissions on backup storage directory"))?;
                }

                let backup_storage_dir = path.canonicalize()
                    .map_err(|e| Error::file_io(e, path, "Backup storage directory is not accessible"))?;

                let all_backup_users = cross::PLATFORM.access().group_users(&backup_users_group)?;
                for subdir in backup_storage_subdirs(&backup_storage_dir) {
                    if !subdir.exists() {
                        fs::create_dir(&subdir)
                            .map_err(|e| Error::file_io(e, &subdir, "Failed to create backup storage subdirectory"))?;
                    }
                    
                    cross::PLATFORM.fs()
                        .own_capable(&subdir, admin_uaid, &backup_users_gaid_capable, BACKUP_STORAGE_DIR_PERMISSIONS)
                        .map_err(|e| Error::file_io(e, &subdir, "Failed to set permissions on backup storage subdirectory"))?;
                    
                    for backup_user in &all_backup_users {
                        let user_subdir = subdir.join(backup_user.username().as_ffi());
                        if !user_subdir.exists() {
                            fs::create_dir(&user_subdir)
                                .map_err(|e| Error::file_io(e, &user_subdir, "Failed to create backup storage subdirectory"))?;
                        }
                        
                        let backup_user_group_capable = cross::PLATFORM.access().user_primary_group(&backup_user)?;
                        cross::PLATFORM.fs()
                            .own_capable(&user_subdir, backup_user, &backup_user_group_capable, BACKUP_STORAGE_DIR_PERMISSIONS)
                            .map_err(|e| Error::file_io(e, &user_subdir, "Failed to set permissions on user backup storage subdirectory"))?;
                    }
                }

                Self::fs_version(&backup_storage_dir).setup(config)?;
            },
            Self::UserConfig { path, .. } => {
                if !path.exists() {
                    let dir = path.parent()
                        .ok_or_else(|| Error::file_io_err(path, "Unable to determine user config directory"))?;
                    fs::create_dir_all(dir)
                        .map_err(|e| Error::file_io(e, path, "Failed to create user config directory"))?;
                    fs::write(path, config::CONFIG_DEFAULTS)
                        .map_err(|e| Error::file_io(e, path, "Failed to create default user config file"))?;
                }
            },
            Self::FileSytemVersion { path, .. } => {
                if !path.exists() {
                    fs::write(path, crate::consts::SOURCETRAIT_BACKUP_FS_VERSION.to_string())
                        .map_err(|e| Error::file_io(e, path, "Failed to create file system version file"))?;
                    cross::PLATFORM.fs()
                        .own_capable(&path, admin_uaid, &backup_users_gaid_capable, BACKUP_STORAGE_FILE_PERMISSIONS)
                        .map_err(|e| Error::file_io(e, &path, "Failed to set permissions on file system version file"))?;
                }
            },
            Self::RemoteStorageDir { remote, storage_dir } => {
                let fs_version_file = SourceTraitBackupPath::fs_version(storage_dir);

                // if the base dir or the version file don't exist ... run a full setup
                if !crate::cmd::ssh_run::ssh_dirs_exist(remote, &backup_storage_check_paths(&storage_dir))? {
                    let dirs = backup_storage_subdirs(storage_dir);
                    crate::cmd::ssh_run::ssh_make_dirs(remote, &dirs)
                        .map_err(|e| Error::remote_cmd(e, remote, &format!(
                            "Unable to create backup storage directory {}", storage_dir.tik_path())))?;

                    crate::cmd::ssh_run::ssh_write_file(remote, &fs_version_file, &crate::consts::SOURCETRAIT_BACKUP_FS_VERSION.to_string())
                        .map_err(|e| Error::remote_cmd(e, remote, &format!(
                            "Unable to write sourcetrait backup filesystem version file {}", &fs_version_file.tik_path())))?;
                } else {
                    // check the remote's sourcetrait backup file system version and throw an error if we need an upgrade
                    let remote_fs_version = crate::cmd::ssh_run::ssh_file_contents(remote, &fs_version_file)?
                        .ok_or_else(|| Error::remote_cmd_err(remote, &format!(
                            "Failed to read remote file system version file {}", &fs_version_file.tik_path())))?;
                    let remote_fs_version = semver::Version::parse(&remote_fs_version)
                        .map_err(|e| Error::remote_cmd(e, remote, &format!(
                            "Failed to parse remote file system version file {}", &fs_version_file.tik_path())))?;

                    if !crate::consts::SOURCETRAIT_BACKUP_FS_VERSION_REQ.matches(&remote_fs_version) {
                        return Err(Error::remote_cmd_err(remote, &format!(
                            "Remote file system version {} does not match version {}. An upgrade is required.",
                            remote_fs_version.to_string().tik_cmd(), crate::consts::SOURCETRAIT_BACKUP_FS_VERSION.to_string().tik_cmd())));
                    } else {
                        // if for some reason there are missing subdirs, attempt to recover: warn and create them
                        let dirs = backup_storage_dirs(storage_dir);
                        if !crate::cmd::ssh_run::ssh_dirs_exist(remote, &dirs)? {
                            //todo: warn
                            crate::cmd::ssh_run::ssh_make_dirs(remote, &dirs)
                                .map_err(|e| Error::remote_cmd(e, remote,
                                    &format!("Failed to create remote backup storage directories {:?}", dirs)))?;
                        }
                    }
                }
            },
            _ => {}
        }

        Ok(())
    }

    fn verify_storage_dir(storage_dir: &Path) -> Result<()> {
        match storage_dir.exists() {
            true => Ok(()),
            false => Err(Error::file_io_err(storage_dir, "Backup storage directory does not exist"))
        }
    }

    fn storage_subdir(&self) -> Option<PathBuf> {
        match self {
            Self::FullBackup{ storage_dir, .. } => Some(storage_dir.join(consts::BACKUP_FULL_DIRNAME)),
            Self::IncrementalBackup{ storage_dir, .. } => Some(storage_dir.join(consts::BACKUP_INCREMENTAL_DIRNAME)),
            Self::Archive{ storage_dir, .. } => Some(storage_dir.join(consts::BACKUP_ARCHIVE_DIRNAME)),
            _ => None
        }
    }

    /// Performs task-based setup of a path if necessary, based on the current job's configuration item.
    /// This includes creating directories and setting permissions.
    /// Currently, this applies to everything but [Bak8Path::StorageDir].
    pub fn prepare(&self, config: &BackupConfig) -> Result<()> {
        let osnap = os_snapshot();
        let uaid = osnap.current_user_aid();
        let gaid_capable = osnap.current_user_primary_group_aid();
        //let admin_uaid = storage_admin_aid(config)?;
        //let backup_users_gaid = backup_users_id(config)?;
        //let backup_users_gaid_capable = cross::Capable::Capable(backup_users_gaid);
        
        match self {
            Self::BackupDir { storage_dir, path, .. }
                | Self::FullBackup { storage_dir, path, .. }
                | Self::IncrementalBackup { storage_dir, path, .. }
                | Self::Archive { storage_dir, path, .. } =>
            {
                Self::verify_storage_dir(storage_dir)?;

                if !path.exists() {
                    let subdir = self.storage_subdir().expect("subdir");
                    let mut last_dir = &subdir;

                    // create context subdir: user
                    let user_subdir = last_dir.join(osnap.current_username().as_str());
                    last_dir = &user_subdir;
                    if !user_subdir.exists() {
                        fs::create_dir(&user_subdir)
                            .map_err(|e| Error::file_io(e, &user_subdir, "Failed to create context subdirectory for user"))?;
                        //unix::fs::chown(&user_subdir, Some(osnap.current_user_aiduser_aid()), Some(group_aid()))
                        cross::PLATFORM.fs()
                            .own_capable(&user_subdir, &uaid, &gaid_capable, BACKUP_RUN_DIR_PERMISSIONS)
                            .map_err(|e| Error::file_io(e, &user_subdir, "Failed to set ownership for user context subdirectory"))?;
                        //fs::set_permissions(&user_subdir, std::os::unix::fs::PermissionsExt::from_mode(BACKUP_RUN_DIR_UNIX_MODE))
                        //    .map_err(|e| Error::file_io(e, &user_subdir, "Failed to set permissions on user context subdirectory"))?;
                    }
                    
                    // create context subdir: host
                    let host_subdir = last_dir.join(osnap.hostname().as_str());
                    last_dir = &host_subdir;
                    if !host_subdir.exists() {
                        fs::create_dir(&host_subdir)
                            .map_err(|e| Error::file_io(e, &host_subdir, "Failed to create context subdirectory for host"))?;
                        cross::PLATFORM.fs()
                            .own_capable(&host_subdir, &uaid, &gaid_capable, BACKUP_STORAGE_DIR_PERMISSIONS)
                            .map_err(|e| Error::file_io(e, &host_subdir, "Failed to set ownership for host context subdirectory"))?;
                    }

                    let datetime = match self {
                        Self::BackupDir { path_parts, .. } if path_parts.datetime.is_some() => path_parts.datetime.as_ref().unwrap(),
                        Self::FullBackup { run_name, .. } => &run_name.datetime,
                        Self::IncrementalBackup { run_name, .. } => &run_name.datetime,
                        Self::Archive { run_name, .. } => &run_name.datetime,
                        _ => return Ok(()),
                    };

                    // create context subdir: year
                    let year_subdir = last_dir.join(datetime.format("%Y").to_string());
                    last_dir = &year_subdir;
                    if !year_subdir.exists() {
                        fs::create_dir(&year_subdir)
                            .map_err(|e| Error::file_io(e, &year_subdir, "Failed to create context subdirectory for year"))?;
                        //unix::fs::chown(&year_subdir, Some(user_aid()), Some(group_aid()))
                        cross::PLATFORM.fs()
                            .own_capable(&year_subdir, &uaid, &gaid_capable, BACKUP_RUN_DIR_PERMISSIONS)
                            .map_err(|e| Error::file_io(e, &year_subdir, "Failed to set ownership for year context subdirectory"))?;
                        //fs::set_permissions(&year_subdir, std::os::unix::fs::PermissionsExt::from_mode(BACKUP_RUN_DIR_UNIX_MODE))
                        //    .map_err(|e| Error::file_io(e, &year_subdir, "Failed to set permissions on year context subdirectory"))?;
                    }

                    // create context subdir: month
                    let month_subdir = last_dir.join(datetime.format("%m").to_string());
                    last_dir = &month_subdir;
                    if !month_subdir.exists() {
                        fs::create_dir(&month_subdir)
                            .map_err(|e| Error::file_io(e, &month_subdir, "Failed to create context subdirectory for month"))?;
                        cross::PLATFORM.fs()
                            .own_capable(&year_subdir, &uaid, &gaid_capable, BACKUP_RUN_DIR_PERMISSIONS)
                            .map_err(|e| Error::file_io(e, &month_subdir, "Failed to set ownership for month context subdirectory"))?;
                        //unix::fs::chown(&month_subdir, Some(user_aid()), Some(group_aid()))
                        //fs::set_permissions(&month_subdir, std::os::unix::fs::PermissionsExt::from_mode(BACKUP_RUN_DIR_UNIX_MODE))
                        //    .map_err(|e| Error::file_io(e, &month_subdir, "Failed to set permissions on month context subdirectory"))?;
                    }

                    // create context subdir: day
                    let day_subdir = last_dir.join(datetime.format("%d").to_string());
                    last_dir = &day_subdir;
                    if !day_subdir.exists() {
                        fs::create_dir(&day_subdir)
                            .map_err(|e| Error::file_io(e, &day_subdir, "Failed to create context subdirectory for day"))?;
                        cross::PLATFORM.fs()
                            .own_capable(&day_subdir, uaid, &gaid_capable, BACKUP_RUN_DIR_PERMISSIONS)
                            .map_err(|e| Error::file_io(e, &day_subdir, "Failed to set ownership for day context subdirectory"))?;
                        //unix::fs::chown(&day_subdir, Some(user_aid()), Some(group_aid()))
                        //fs::set_permissions(&day_subdir, std::os::unix::fs::PermissionsExt::from_mode(BACKUP_RUN_DIR_UNIX_MODE))
                        //    .map_err(|e| Error::file_io(e, &day_subdir, "Failed to set permissions on day context subdirectory"))?;
                    }
               }

                Ok(())
            },
            Self::StorageDir(_) => Ok(()),
            _ => Ok(())
        }
    }

    pub fn backup_run_name(&self) -> Option<&BackupRunName> {
        match self {
            Self::FullBackup{run_name, ..} => Some(run_name),
            Self::IncrementalBackup{run_name, ..} => Some(run_name),
            Self::Archive{run_name, ..} => Some(run_name),
            Self::Log{run_name, ..} => Some(run_name),
            _ => None,
        }
    }

    pub fn as_path(&self) -> &Path {
        match self {
            Self::StorageDir(storage_dir) => storage_dir,
            Self::BackupDir{path, ..} => path,
            Self::FullBackup{path, ..} => path,
            Self::IncrementalBackup{path, ..} => path,
            Self::Archive{path, ..} => path,
            Self::Log{path, ..} => path,
            Self::UserConfig{path, ..} => path,
            Self::FileSytemVersion { path, .. } => path,
            Self::RemoteStorageDir { storage_dir, .. } => storage_dir,
        }
    }

    pub fn to_path_buf(&self) -> PathBuf {
        self.as_path().to_path_buf()
    }
}

impl From<SourceTraitBackupPath> for PathBuf {
    fn from(value: SourceTraitBackupPath) -> Self {
        value.to_path_buf()
    }
}

pub fn expand_path(path_str: &str) -> Result<PathBuf> {
    Ok(crate::sys::expand_env(path_str)?.to_string().into())
}

pub fn home_dir() -> Result<PathBuf> {
    let home: PathBuf = if let Ok(sourcetrait_backup_home) = std::env::var(consts::ENV_SOURCETRAIT_BACKUP_HOME) {
        PathBuf::from(sourcetrait_backup_home)
    } else {
        option_env!("HOME")
            .ok_or_else(|| Error::FileIO {
                message: "Environment variable is not set: $HOME".to_string(),
                path: "$HOME".to_string(),
                cause: None})?
            .into()
    };

    Ok(home)
}

pub fn setup_home_config(force: bool) -> Result<()> {
    let home_config_dir = home_dir()?.join(consts::HOME_CONFIG_DIR);
    let home_config_file = home_config_dir.join(consts::SOURCETRAIT_BACKUP_CONFIG_FILENAME);

    if !home_config_dir.exists() {
        fs::create_dir_all(&home_config_dir)
            .map_err(|e| Error::file_io(e, &home_config_dir, "Unable to create user config directory"))?;
    }

    if !home_config_file.exists() || force {
        fs::write(&home_config_file, config::CONFIG_DEFAULTS)
            .map_err(|e| Error::file_io(e, &home_config_file, "Unable to create default user config file"))?;
    }

    Ok(())
}

pub fn backup_storage_check_paths(backup_storage_dir: &Path) -> Vec<PathBuf> {
    vec![
        backup_storage_dir.to_path_buf(),
        backup_storage_dir.join(consts::SOURCETRAIT_BACKUP_FS_VERSION_FILENAME),
    ]
}

pub fn backup_storage_dirs(backup_storage_dir: &Path) -> Vec<PathBuf> {
    vec![
        backup_storage_dir.to_path_buf(),
        backup_storage_dir.join(consts::BACKUP_ARCHIVE_DIRNAME),
        backup_storage_dir.join(consts::BACKUP_FULL_DIRNAME),
        backup_storage_dir.join(consts::BACKUP_INCREMENTAL_DIRNAME),
        backup_storage_dir.join(consts::BACKUP_LOGS_DIRNAME)
    ]
}

pub fn backup_storage_subdirs(backup_storage_dir: &Path) -> Vec<PathBuf> {
    vec![
        backup_storage_dir.join(consts::BACKUP_ARCHIVE_DIRNAME),
        backup_storage_dir.join(consts::BACKUP_FULL_DIRNAME),
        backup_storage_dir.join(consts::BACKUP_INCREMENTAL_DIRNAME),
        backup_storage_dir.join(consts::BACKUP_LOGS_DIRNAME)
    ]
}

pub fn verify_backup_dirs(config: &config::BackupConfig) -> Result<()> {
    let backup_storage_dir = &config.backup_storage_dir_path();
    let backup_storage_dir = backup_storage_dir.canonicalize()
        .map_err(|e| Error::configured_dir(&backup_storage_dir, config::consts::CFG_BACKUP_STORAGE_DIR, e))?;

    for subdir in backup_storage_subdirs(&backup_storage_dir) {
        subdir.canonicalize()
            .map_err(|e| Error::configured_subdir(&subdir, config::consts::CFG_BACKUP_STORAGE_DIR, e))?;
    }

    Ok(())
}

pub fn with_sha256_extension(filepath: &std::path::Path) -> PathBuf {
    filepath.with_extension(format!("{}.{}", filepath.extension().unwrap().to_string_lossy(), consts::SHA256_EXTENSION))
}
