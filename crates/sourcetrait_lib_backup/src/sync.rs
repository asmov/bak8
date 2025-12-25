mod backup;
mod archive;

#[derive(Debug, Clone)]
pub struct Remote {
    pub name: String,
    pub host: String,
    pub user: Option<String>,
    pub platform: SyncPlatform
}

impl Remote {
    pub fn addr(&self) -> String {
        if let Some(user) = &self.user {
            format!("{}@{}", user, self.host)
        } else {
            self.host.clone()
        }
    }
}

impl From<crate::config::BackupConfigRemote> for Remote {
    fn from(remote: crate::config::BackupConfigRemote) -> Self {
        Self {
            name: remote.name,
            host: remote.host,
            user: remote.user,
            platform: remote.platform
        }
    }
}

impl From<&crate::config::BackupConfigRemote> for Remote {
    fn from(remote: &crate::config::BackupConfigRemote) -> Self {
        Self {
            name: remote.name.clone(),
            host: remote.host.clone(),
            user: remote.user.clone(),
            platform: remote.platform
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncPlatform {
    #[default]
    GNU
}


pub use backup::{SyncBackupJob, SyncBackupJobOutput};
pub use archive::{SyncArchiveJob, SyncArchiveJobOutput};