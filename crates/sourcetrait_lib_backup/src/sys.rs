//! # Users and Group Permissions
//! - Core storage directories are owned by root:bakusr if the group exists, otherwise current_user:current_user. Mode: 750
//! - Context directories (host, user, etc.):
//!   - host: Owned by root:bakusr if the group exists, otherwise current_user:current_user. Mode: 750
//!   - user: Owned by user:user. Mode: 700

use std::{sync::{Arc, Mutex, OnceLock}};
use uzers::{self, get_current_groupname, Groups, Users};
use crate::{error::*, config::*};
use crate::*;

pub fn hostname() -> &'static str {
    static HOSTNAME: OnceLock<String> = OnceLock::new();
    &HOSTNAME.get_or_init(|| whoami::hostname().unwrap())
}

pub fn username() -> &'static str {
    static USERNAME: OnceLock<String> = OnceLock::new();
    &USERNAME.get_or_init(|| {
        let cache_lock = users_cache().lock().unwrap();
        cache_lock.get_current_username().unwrap().to_string_lossy().to_string()
    })
}

pub fn usergroup() -> &'static str {
    static GROUPNAME: OnceLock<String> = OnceLock::new();
    &GROUPNAME.get_or_init(|| {
        let cache_lock = users_cache().lock().unwrap();
        cache_lock.get_current_groupname().unwrap().to_string_lossy().to_string()
    })
}

fn users_cache() -> &'static Arc<Mutex<uzers::UsersCache>> {
    static CACHE: OnceLock<Arc<Mutex<uzers::UsersCache>>> = OnceLock::new();
    &CACHE.get_or_init(|| Arc::new(Mutex::new(uzers::UsersCache::new())))
}

pub const GROUP_BACKUP_USERNAME: &'static str = "bakusr";

/// Returns UID 0 (root) if the `bakusr` group exists, otherwise the current user's UID.
pub fn storage_admin_uid(config: &BackupConfig) -> Result<u32> {
    static UID: OnceLock<Option<u32>> = OnceLock::new();
    let uid = UID.get_or_init(|| {
        let Ok(user) = &config.get_storage_admin_user() else {
            return None;
        };

        let cache_lock = users_cache().lock().unwrap();
        match cache_lock.get_user_by_name(user as &str) {
            Some(ref user) => Some(user.uid()),
            None => None,
        }
    });

    uid.ok_or_else(|| Error::AccountNotFound { account_type: Error::ACCOUNT_USER, account: config.storage_admin_user.clone()})
}

/// Returns the GID for the `bakusr` group if it exists, otherwise None.
pub fn backup_users_gid(config: &BackupConfig) -> Result<u32> {
    static GID: OnceLock<Option<u32>> = OnceLock::new();
    let gid = GID.get_or_init(|| {
        let Ok(group) = &config.get_backup_users_group() else {
            return None;
        };

        let cache_lock = users_cache().lock().unwrap();
        match cache_lock.get_group_by_name(group as &str) {
            Some(ref group) => Some(group.gid()),
            None => None,
        }
    });

    gid.ok_or_else(|| Error::AccountNotFound { account_type: Error::ACCOUNT_GROUP, account: config.backup_users_group.clone()})
}

pub fn uid() -> u32 {
    static UID: OnceLock<u32> = OnceLock::new();
    *UID.get_or_init(|| {
        let cache_lock = users_cache().lock().unwrap();
        cache_lock.get_current_uid()
    })
}

pub fn gid() -> u32 {
    static GID: OnceLock<u32> = OnceLock::new();
    *GID.get_or_init(|| {
        let cache_lock = users_cache().lock().unwrap();
        cache_lock.get_current_gid()
    })
}

/// We perform a custom lookup for $GROUP if it can't be expanded
pub fn expand_env(s: &str) -> Result<Cow<'_, str>> {
    let expanded = shellexpand::full_with_context(
        s,
        || cross::PLATFORM.path().home_dir().ok().and_then(|p| p.into_os_string().into_string().ok()),
        |var| {
            match var {
                "GROUP" => env::var("GROUP").map(Option::Some)
                    .or_else(|e| {
                        get_current_groupname()
                            .ok_or(e)
                            .map(|s| s.to_str().and_then(|s| Some(s.to_string())))
                    }),
                var => env::var(var).map(Option::Some).or_else(|_| Ok(None)),
            }
        }
    );
    
    match expanded {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::Generic { msg: format!("Unable to expand environment variables for: {s} :: {}", e) })
    }
}
