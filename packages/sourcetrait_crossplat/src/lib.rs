pub mod command;
pub mod cross {
    pub mod cross;
    pub mod standard;
    pub mod platform;
}
pub mod dir {
    pub mod cross;
    pub mod xdg;
}
pub mod error;
pub mod os {
    //#[cfg(target_os = "linux")]
    pub mod linux {
        pub mod consts;
        pub mod cross;
    }
    //#[cfg(target_os = "macos")]
    pub mod macos {
        pub mod consts;
        pub mod cross;
    }
    pub mod unixy {
        pub mod common;
        pub mod consts;
    }
    pub mod unsupported {
        pub mod consts;
        pub mod cross;
    }
    //#[cfg(target_os = "windows")]
    pub mod windows {
        pub mod consts;
        pub mod cross;
    }
}

pub(crate) use std::{
    borrow::Cow,
    env,
    fs,
    path::{Path, PathBuf},
    process::{self, Command},
    sync::{Arc, Mutex, MutexGuard, OnceLock},
};
pub(crate) use uzers::Groups;
pub(crate) use self::{
    cross::standard::*,
};

pub use self::{
    command::*,
    cross::{
        cross::*,
        platform::*,
    },
    dir::{
        cross::*,
        xdg::*,
    },
    error::*,
};
//#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) mod unixy {
    pub(crate) use crate::os::unixy::{
        consts::*,
        common::*,
    };
}
//#[cfg(target_os = "linux")]
pub(crate) mod linux {
    #[allow(unused)]
    pub(crate) use crate::os::linux::{
        consts::*,
        cross::*,
    };
}
//#[cfg(target_os = "macos")]
pub(crate) mod macos {
    pub(crate) use crate::os::macos::{
        consts::*,
        cross::*,
    };
}
//#[cfg(target_os = "windows")]
pub(crate) mod windows {
    #[allow(unused)]
    pub(crate) use crate::os::macos::{
        consts::*,
        cross::*,
    };
}

pub mod prelude {
    pub use crate::{
        CrossDir,
        CrossError,
        CrossResult,
        CrossPlatform,
        XdgDir,
    };
}








