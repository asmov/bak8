//! # Users and Group Permissions
//! - Core storage directories are owned by root:bakusr if the group exists, otherwise current_user:current_user. Mode: 750
//! - Context directories (host, user, etc.):
//!   - host: Owned by root:bakusr if the group exists, otherwise current_user:current_user. Mode: 750
//!   - user: Owned by user:user. Mode: 700

use crate::{error::*, config::*};
use crate::*;

pub fn hostname() -> &'static str {
    static HOSTNAME: LazyLock<Arc<String>> = LazyLock::new(|| {
        cross::PLATFORM.net().hostname()
            .expect("Failed to determine hostname")
    });
    
    &HOSTNAME
}

pub fn username() -> &'static str {
    static USERNAME: LazyLock<String> = LazyLock::new(|| {
        cross::PLATFORM.access().current_user()
            .expect("Failed to determine current username")
            .username()
            .try_into_utf8()
            .expect("Current username is not UTF8")
    });
    
    &USERNAME
}

pub fn groupname() -> cross::Capable<cross::PrimaryUserGroupsCapable, &'static str> {
    static GROUPNAME: LazyLock<cross::Capable<cross::PrimaryUserGroupsCapable, String>> = LazyLock::new(|| {
        let access = cross::PLATFORM.access();
        let user = access.current_user()
            .expect("Failed to determine current user");
        access.user_primary_group(&user)
            .expect("Failed to lookup current primary user")
            .map_into(|g| g.groupname().try_into_utf8().expect("Current groupname is not UTF8"))
    });
    
    GROUPNAME.as_deref()
}

pub const GROUP_BACKUP_USERNAME: &'static str = "bakusr";

/// Returns UID 0 (root) if the `bakusr` group exists, otherwise the current user's UID.
pub fn storage_admin_aid(config: &BackupConfig) -> Result<cross::AID<'static>> {
    static AID: OnceLock<Option<cross::AccessID>> = OnceLock::new();
    let aid = AID.get_or_init(|| {
        let Ok(user) = &config.get_storage_admin_user() else {
            return None;
        };

        let user = cross::PLATFORM.access()
            .user(cross::AID::Name(TwoStr::new_utf8(user as &str)))
            .expect("Unable to lookup user");
        
        match user {
            Some(u) => Some(u.id()),
            None => None,
        }
    });

    aid.as_ref().map(|id| id.as_ref())
        .ok_or_else(|| Error::AccountNotFound { account_type: Error::ACCOUNT_USER, account: config.storage_admin_user.clone()})
}

/// Returns the GID for the `bakusr` group if it exists, otherwise None.
pub fn backup_users_id(config: &BackupConfig) -> Result<cross::AID<'static>> {
    static AID: OnceLock<Option<cross::AccessID>> = OnceLock::new();
    let aid = AID.get_or_init(|| {
        let Ok(group) = &config.get_backup_users_group() else {
            return None;
        };

        let group = cross::PLATFORM.access()
            .group(cross::AID::Name(TwoStr::new_utf8(group as &str)))
            .expect("Unable to lookup group");
        
        match group {
            Some(g) => Some(g.id()),
            None => None,
        }
    });

    aid.as_ref().map(|aid| aid.as_ref())
        .ok_or_else(|| Error::AccountNotFound { account_type: Error::ACCOUNT_GROUP, account: config.backup_users_group.clone()})
}

pub fn user_aid() -> cross::AID<'static> {
    static UID: OnceLock<cross::AccessID> = OnceLock::new();
    let aid = UID.get_or_init(|| {
        let user = cross::PLATFORM.access()
            .current_user()
            .expect("Failed to determine current user");
        
        user.id()
    });
    
    aid.as_ref()
}

pub fn group_aid() -> cross::Capable<cross::PrimaryUserGroupsCapable, cross::AID<'static>> {
    static AID: OnceLock<cross::Capable<cross::PrimaryUserGroupsCapable, cross::AccessID>> = OnceLock::new();
    let aid = AID.get_or_init(|| {
        let user = cross::PLATFORM.access()
            .current_user()
            .expect("Failed to determine current user");
        
        let group = cross::PLATFORM.access()
            .user_primary_group(&user)
            .expect("Failed to lookup current primary group");
        
        group.map(cross::UserGroup::id)
    });
    
    aid.as_ref().map(|aid| aid.as_ref())
}

/// We perform a custom lookup for $GROUP if it can't be expanded
pub fn expand_env(s: &str) -> Result<Cow<'_, str>> {
    let expanded = shellexpand::full_with_context(
        s,
        || cross::PLATFORM.path().home_dir().ok().and_then(|p| p.into_os_string().into_string().ok()),
        |var| {
            match var {
                "GROUP" => env::var("GROUP").map(Option::Some)
                    .or_else(|_| {
                        groupname()
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
