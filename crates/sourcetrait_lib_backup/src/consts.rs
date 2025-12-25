use crate::*;

pub const ENV_SOURCETRAIT_BACKUP_HOME: &'static str = "BACKUP_HOME";
pub const BACKUP_ARCHIVE_DIRNAME: &'static str = "archive";
pub const TAR_XZ_EXTENSION: &'static str = "tar.xz";
pub const SHA256_EXTENSION: &str = "sha256";

pub(crate) const SOURCETRAIT_BACKUP: &'static str = "backup";
pub(crate) const SOURCETRAIT_BACKUP_FS_VERSION: semver::Version = semver::Version::new(1, 0, 0);
pub(crate) const HOME_CONFIG_DIR: &'static str = ".config/sourcetrait/backup";
pub(crate) const SOURCETRAIT_BACKUP_CONFIG_FILENAME: &'static str = "backup.toml";
pub(crate) const BACKUP_FULL_DIRNAME: &'static str = "full";
pub(crate) const BACKUP_INCREMENTAL_DIRNAME: &'static str = "incremental";
pub(crate) const BACKUP_LOGS_DIRNAME: &'static str = "logs";
pub(crate) const SOURCETRAIT_BACKUP_FS_VERSION_FILENAME: &'static str = ".backup_fs_version";
pub(crate) const CFG_BACKUP_STORAGE_DIR: &'static str = "backup_storage_dir";

pub(crate) static SOURCETRAIT_BACKUP_FS_VERSION_REQ: LazyLock<semver::VersionReq> = LazyLock::new(|| {
    semver::VersionReq::parse("^1").expect("^1")
});

