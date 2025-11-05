use crate::*;

pub(crate) fn std_copy_file<P1, P2>(src: P1, dest: P2) -> CrossResult<()>
where
    P1: AsRef<Path> + Into<PathBuf>,
    P2: AsRef<Path> + Into<PathBuf>,
{
    fs::copy(&src, &dest)
        .map(|_bytes| ())
        .map_err(|source| CrossError::io(ErrIo::copy_file(src, dest), source)) 
}

pub(crate) fn expand_env(s: &str) -> CrossResult<Cow<'_, str>> {
    shellexpand::env(s)
        .map_err(|source| CrossError::env_var(source.var_name, source.cause))
}

pub(crate) fn users_cache() -> &'static Arc<Mutex<uzers::UsersCache>> {
    static CACHE: OnceLock<Arc<Mutex<uzers::UsersCache>>> = OnceLock::new();
    &CACHE.get_or_init(|| Arc::new(Mutex::new(uzers::UsersCache::new())))
}

pub(crate) fn users_cache_lock<'lock>() -> MutexGuard<'lock, uzers::UsersCache> {
    users_cache().lock().expect("users cache")
}

pub(crate) fn primary_user_group() -> CrossResult<String> {
    let name = {
        let users_cache = users_cache_lock();
        users_cache.get_current_groupname()
            .map(|s| 
                s.to_str()
                    .map(|s| Ok(s.to_string()))
                    .ok_or_else(|| CrossError::not_found(ErrNotFound::UserGroup))?
            )
            .ok_or_else(|| CrossError::not_found(ErrNotFound::UserGroup))?
    };
    
    name
}

pub(crate) fn base_dirs_cache() -> &'static Arc<Mutex<directories::BaseDirs>> {
    static CACHE: OnceLock<Arc<Mutex<directories::BaseDirs>>> = OnceLock::new();
    &CACHE.get_or_init(|| Arc::new(Mutex::new(directories::BaseDirs::new().expect("Unsupported base directories"))))
}

pub(crate) fn base_dirs_lock<'lock>() -> MutexGuard<'lock, directories::BaseDirs> {
    base_dirs_cache().lock().expect("base dirs cache")
}

pub(crate) fn home_dir() -> CrossResult<PathBuf> {
    let dir = {
        let base_dirs_lock = base_dirs_lock();
        base_dirs_lock.home_dir().to_path_buf()
    };
    
    Ok(dir)
}

pub(crate) fn init_xdg_dir_for<P>(base: XdgDir, subdir: P) -> CrossResult<PathBuf>
where
    P: AsRef<Path> + Into<PathBuf>
{
    let dir = {
        let base_dirs_lock = base_dirs_lock();
        let home_dir = base_dirs_lock.home_dir();
        base.join_homed(home_dir, subdir)?
    };
    
    fs::create_dir_all(&dir)
        .map_err(|source| CrossError::io(ErrIo::CreateDir(dir.clone()), source))?;
    
    Ok(dir)
}

pub(crate) fn xdg_dir_for<P>(base: XdgDir, subdir: P) -> CrossResult<PathBuf>
where
    P: AsRef<Path> + Into<PathBuf>
{
    let dir = {
        let base_dirs_lock = base_dirs_lock();
        let home_dir = base_dirs_lock.home_dir();
        base.join_homed(home_dir, subdir)?
    };
    
    Ok(dir)
}

pub(crate) fn init_dir_for<P>(base: CrossDir, subdir: P) -> CrossResult<PathBuf>
where
    P: AsRef<Path> + Into<PathBuf>
{
    let base_dir = {
        let base_dirs_lock = base_dirs_lock();
        
        match base {
            CrossDir::HomeCache => base_dirs_lock.cache_dir().to_path_buf(),
            CrossDir::HomeConfig => base_dirs_lock.config_dir().to_path_buf(),
            CrossDir::HomeData => base_dirs_lock.data_dir().to_path_buf(),
            CrossDir::HomeState => base_dirs_lock.state_dir().map(|d| d.to_path_buf())
                .ok_or_else(|| CrossError::unsupported_dir(base))?,
        }
    };
    
    fs::create_dir_all(&base_dir)
        .map_err(|source| CrossError::io(ErrIo::CreateDir(base_dir.clone()), source))?;
    
    //review: should we handle chmod/chown for the base dir before moving on?
    
    let dir = base_dir.join(subdir);
    fs::create_dir_all(&dir)
        .map_err(|source| CrossError::io(ErrIo::CreateDir(dir.clone()), source))?;
    
    Ok(dir)
}

pub(crate) fn dir_for<P>(base: CrossDir, subdir: P) -> CrossResult<PathBuf>
where
    P: AsRef<Path> + Into<PathBuf>
{
    let dir = {
        let base_dirs_lock = base_dirs_lock();
        
        match base {
            CrossDir::HomeCache => Ok(base_dirs_lock.cache_dir().join(subdir)),
            CrossDir::HomeConfig => Ok(base_dirs_lock.config_dir().join(subdir)),
            CrossDir::HomeData => Ok(base_dirs_lock.data_dir().join(subdir)),
            CrossDir::HomeState => base_dirs_lock.state_dir().map(|d| d.join(subdir))
                .ok_or_else(|| CrossError::unsupported_dir(base)),
        }
    };
    
    dir
}
