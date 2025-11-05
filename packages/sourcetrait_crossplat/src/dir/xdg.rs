use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XdgDir {
    HomeConfig,
    HomeCache,
    HomeData,
    HomeState,
}

impl XdgDir {
    const ENV_CONFIG_HOME: &'static str = "XDG_CONFIG_HOME";
    const ENV_CACHE_HOME: &'static str = "XDG_CACHE_HOME";
    const ENV_DATA_HOME: &'static str = "XDG_DATA_HOME";
    const ENV_STATE_HOME: &'static str = "XDG_STATE_HOME";
    
    const ENV_VAR_CONFIG_HOME: &'static str = "$XDG_CONFIG_HOME";
    const ENV_VAR_CACHE_HOME: &'static str = "$XDG_CACHE_HOME";
    const ENV_VAR_DATA_HOME: &'static str = "$XDG_DATA_HOME";
    const ENV_VAR_STATE_HOME: &'static str = "$XDG_STATE_HOME";
    
    const DEFAULT_CONFIG_HOME: &'static str = ".config";
    const DEFAULT_CACHE_HOME: &'static str = ".cache";
    const DEFAULT_DATA_HOME: &'static str = ".local/share";
    const DEFAULT_STATE_HOME: &'static str = ".local/state";
}
    
impl XdgDir {
    pub const fn env_var_name(&self) -> &'static str {
        match self {
            Self::HomeConfig => Self::ENV_CONFIG_HOME,
            Self::HomeCache => Self::ENV_CACHE_HOME,
            Self::HomeData => Self::ENV_DATA_HOME,
            Self::HomeState => Self::ENV_STATE_HOME,
        }
    }
    
    pub const fn env_var(&self) -> &'static str {
        match self {
            Self::HomeConfig => Self::ENV_VAR_CONFIG_HOME,
            Self::HomeCache => Self::ENV_VAR_CACHE_HOME,
            Self::HomeData => Self::ENV_VAR_DATA_HOME,
            Self::HomeState => Self::ENV_VAR_STATE_HOME,
        }
    }
    
    pub fn homed(&self, home: &Path) -> CrossResult<PathBuf> {
        let path = match self {
            Self::HomeCache => env::var_os(Self::ENV_CACHE_HOME).map(PathBuf::from).unwrap_or_else(|| home.join(Self::DEFAULT_CACHE_HOME)),
            Self::HomeConfig => env::var_os(Self::ENV_CONFIG_HOME).map(PathBuf::from).unwrap_or_else(|| home.join(Self::DEFAULT_CONFIG_HOME)),
            Self::HomeData => env::var_os(Self::ENV_DATA_HOME).map(PathBuf::from).unwrap_or_else(|| home.join(Self::DEFAULT_DATA_HOME)),
            Self::HomeState => env::var_os(Self::ENV_STATE_HOME).map(PathBuf::from).unwrap_or_else(|| home.join(Self::DEFAULT_STATE_HOME)),
        };
        
        // avoid shell-expansion when possible        
        let lossy = path.to_string_lossy();  
        if lossy.contains(['$', '~']) {
            shellexpand::full(lossy.as_ref())
                .map(|p| PathBuf::from(p.as_ref()))
                .map_err(|e| CrossError::env_var(e.var_name, e.cause))
        } else {
            Ok(path)
        }
    }
    
    pub fn join_homed<P2>(&self, home: &Path, subdir: P2) -> CrossResult<PathBuf>
    where
        P2: AsRef<Path> + Into<PathBuf>,
    {
        Ok(self.homed(home)?.join(subdir))
    }
    
    pub const fn is_default_homed(&self) -> bool {
        match self {
            Self::HomeCache => true,
            Self::HomeConfig => true,
            Self::HomeData => true,
            Self::HomeState => true,
        }
    }
}

impl From<XdgDir> for CrossDir {
    fn from(value: XdgDir) -> Self {
        match value {
            XdgDir::HomeCache => Self::HomeCache,
            XdgDir::HomeConfig => Self::HomeConfig,
            XdgDir::HomeData => Self::HomeData,
            XdgDir::HomeState => Self::HomeState,
        }
    }
}