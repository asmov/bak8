//! # Users and Group Permissions
//! - Core storage directories are owned by root:bakusr if the group exists, otherwise current_user:current_user. Mode: 750
//! - Context directories (host, user, etc.):
//!   - host: Owned by root:bakusr if the group exists, otherwise current_user:current_user. Mode: 750
//!   - user: Owned by user:user. Mode: 700

use crate::*;
use crate::{error::*, config::*};

pub const GROUP_BACKUP_USERNAME: &'static str = "bakusr";

/// Returns UID 0 (root) if the `bakusr` group exists, otherwise the current user's UID.
pub fn storage_admin_aid(config: &BackupConfig) -> Result<cross::AccessIdRef<'static>> {
    static AID: OnceLock<Option<cross::AccessId>> = OnceLock::new();
    let aid = AID.get_or_init(|| {
        let Ok(user) = &config.get_storage_admin_user() else {
            return None;
        };

        let user = cross::PLATFORM.access()
            .user(cross::AccessKeyRef::Name(TwoStr::new_utf8(user as &str)))
            .expect("Unable to lookup user");
        
        match user {
            Some(u) => Some(u.to_id()),
            None => None,
        }
    });

    aid.as_ref().map(|id| id.as_aid())
        .ok_or_else(|| Error::AccountNotFound { account_type: Error::ACCOUNT_USER, account: config.storage_admin_user.clone()})
}

/// Returns the GID for the `bakusr` group if it exists, otherwise None.
pub fn backup_users_id(config: &BackupConfig) -> Result<cross::AccessIdRef<'static>> {
    static AID: OnceLock<Option<cross::AccessId>> = OnceLock::new();
    let aid = AID.get_or_init(|| {
        let Ok(group) = &config.get_backup_users_group() else {
            return None;
        };

        let group = cross::PLATFORM.access()
            .group(cross::AccessKeyRef::Name(TwoStr::new_utf8(group as &str)))
            .expect("Unable to lookup group");
        
        match group {
            Some(g) => Some(g.to_id()),
            None => None,
        }
    });

    aid.as_ref().map(|aid| aid.as_aid())
        .ok_or_else(|| Error::AccountNotFound { account_type: Error::ACCOUNT_GROUP, account: config.backup_users_group.clone()})
}

pub fn user_aid() -> cross::AccessIdRef<'static> {
    static UID: OnceLock<cross::AccessId> = OnceLock::new();
    let aid = UID.get_or_init(|| {
        let user = cross::PLATFORM.access()
            .current_user()
            .expect("Failed to determine current user");
        
        user.to_id()
    });
    
    aid.as_aid()
}

pub fn group_aid() -> cross::Capable<cross::PrimaryUserGroupsCapable, cross::AccessIdRef<'static>> {
    static AID: OnceLock<cross::Capable<cross::PrimaryUserGroupsCapable, cross::AccessId>> = OnceLock::new();
    let aid = AID.get_or_init(|| {
        let user = cross::PLATFORM.access()
            .current_user()
            .expect("Failed to determine current user");
        
        let group = cross::PLATFORM.access()
            .user_primary_group(&user)
            .expect("Failed to lookup current primary group");
        
        group.map(cross::UserGroup::to_id)
    });
    
    aid.as_ref().map(|aid| aid.as_aid())
}

/// We perform a custom lookup for $GROUP if it can't be expanded
pub fn expand_env(s: &str) -> Result<Cow<'_, str>> {
    const GROUP: &'static str = "GROUP";
    let groupname = os_snapshot().current_user_primary_groupname();
    
    let expanded = shellexpand::full_with_context(
        s,
        || {
            cross::PLATFORM.path().home_dir().ok()
                .and_then(|p| p.into_os_string().into_string().ok())
        },
        |var| {
            match var {
                GROUP => env::var(GROUP).map(Option::Some)
                    .or_else(|_| {
                        groupname
                            .ok()
                            .map(|s| Some(s.to_string()))
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
