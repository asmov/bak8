use crate::*;

#[derive(Debug)]
pub(crate) struct OsSnapshot {
    hostname: Arc<String>,
    current_user: Arc<cross::User>,
    current_user_aid: Arc<cross::AccessID>,
    current_username: Arc<String>,
    current_user_primary_group: cross::Capable<cross::PrimaryUserGroupsCapable, Arc<cross::UserGroup>>,
    current_user_primary_group_aid: cross::Capable<cross::PrimaryUserGroupsCapable, Arc<cross::AccessID>>,
    current_user_primary_groupname: cross::Capable<cross::PrimaryUserGroupsCapable, Arc<String>>,
}

impl OsSnapshot {
    pub(crate) fn hostname(&self) -> Arc<String> {
        Arc::clone(&self.hostname)
    }
    
    pub(crate) fn current_user(&self) -> Arc<cross::User> {
        Arc::clone(&self.current_user)
    }
    
    pub(crate) fn current_user_aid(&self) -> Arc<cross::AccessID> {
        Arc::clone(&self.current_user_aid)
    }
    
    pub(crate) fn current_username(&self) -> Arc<String> {
        Arc::clone(&self.current_username)
    }

    pub(crate) fn current_user_primary_group(&self) -> cross::Capable<cross::PrimaryUserGroupsCapable, Arc<cross::UserGroup>> {
        self.current_user_primary_group.map(Arc::clone)
    }
    
    pub(crate) fn current_user_primary_group_aid(&self) -> cross::Capable<cross::PrimaryUserGroupsCapable, Arc<cross::AccessID>> {
        self.current_user_primary_group_aid.map(Arc::clone)
    }
    
    pub(crate) fn current_user_primary_groupname(&self) -> cross::Capable<cross::PrimaryUserGroupsCapable, Arc<String>> {
        self.current_user_primary_groupname.map(Arc::clone)
    }
}

#[derive(Debug)]
pub(crate) struct OsSnapshotInit {
}

pub const E_INIT: &'static str = "os snapshot initialized";

#[inline]
pub(crate) fn os_snapshot() -> Arc<OsSnapshot> {
    os_snapshot_actual(None).expect(E_INIT)
}

pub(crate) fn os_snapshot_actual(init: Option<Box<OsSnapshotInit>>) -> Result<Arc<OsSnapshot>> {
    static SNAPSHOT: OnceLock<Arc<OsSnapshot>> = OnceLock::new();
    
    if init.is_none() {
        return Ok(Arc::clone(SNAPSHOT.get().expect(E_INIT)));
    }
    
    let _init = init.expect(E_INIT);
    
    let hostname = osnap_init_hostname()?;
    let current_user = osnap_init_current_user()?;
    let current_user_aid = Arc::new(current_user.id());
    let current_username = Arc::new(current_user.username().try_into_utf8()?);
    let current_user_primary_group = osnap_init_current_user_primary_group(&current_user)?;
    let current_user_primary_group_aid = current_user_primary_group
        .map(|g| Arc::new(g.id()));
    let current_user_primary_groupname = current_user_primary_group
        .map(|g| g.groupname().try_into_utf8().map(Arc::new))
        .transpose()?;

    let globals = OsSnapshot {
        hostname,
        current_user,
        current_user_aid,
        current_username,
        current_user_primary_group,
        current_user_primary_group_aid,
        current_user_primary_groupname,
    };
    
    SNAPSHOT.get_or_init(|| Arc::new(globals));
    
    Ok(Arc::clone(SNAPSHOT.get().expect(E_INIT)))
}

fn osnap_init_hostname() -> Result<Arc<String>> {
    let hostname = cross::PLATFORM.net().hostname()?;
    Ok(hostname)
}

fn osnap_init_current_user() -> Result<Arc<cross::User>> {
    let user: cross::User = cross::PLATFORM.access().current_user()?;
    Ok(Arc::new(user))
}

fn osnap_init_current_user_primary_group(user: &cross::User) -> Result<cross::Capable<cross::PrimaryUserGroupsCapable, Arc<cross::UserGroup>>> {
    let group = cross::PLATFORM.access()
        .user_primary_group(&user)?
        .map_into(Arc::new);
    
    Ok(group)
}

