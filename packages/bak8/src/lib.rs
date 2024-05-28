pub mod error;
#[macro_use]
pub mod config;
#[macro_use]
pub mod log;
pub mod schedule;
pub mod paths;
pub mod cli;
pub mod cmd;
pub mod run;
pub mod backup;
pub mod job;
pub mod archive;
pub mod sync;
pub mod sys;

pub use run::run_main as run;
pub use error::{Error, Result};
pub use archive::ArchiveJobOutput;
pub use backup::{BackupType, BackupRunName, BackupJobOutput};
pub use paths::Bak8Path;
pub use job::JobOutput;
pub use sync::{Remote, Platform, SyncBackupJobOutput, SyncArchiveJobOutput};

pub mod consts {
    pub const BAK8: &'static str = "bak8";
    pub const BAK8_FS_VERSION: semver::Version = semver::Version::new(1, 0, 0);

    lazy_static::lazy_static! {
        pub static ref BAK8_FS_VERSION_REQ: semver::VersionReq = semver::VersionReq::parse("^1").unwrap();
    }
}
