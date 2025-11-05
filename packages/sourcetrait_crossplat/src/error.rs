use crate::*;

#[derive(Debug, snafu::Snafu)]
pub enum CrossError {
    Io { op: ErrIo, source: std::io::Error },
    NotFound { kind: ErrNotFound },
    EnvVar { var: String, source: std::env::VarError },
    CmdCall { kind: CmdKind, source: std::io::Error },
    Cmd { kind: CmdKind, output: std::process::Output },
    CmdChecked { kind: CmdKind, check: CmdCheck, output: std::process::Output },
    Unsupported,
    UnsupportedDir { kind: CrossDir },
}

pub type CrossResult<T> = Result<T, CrossError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrIo {
    CreateFile(PathBuf),
    CreateDir(PathBuf),
    ReadFile(PathBuf),
    WriteFile(PathBuf),
    ReadDir(PathBuf),
    WriteDir(PathBuf),
    DeleteFile(PathBuf),
    DeleteDir(PathBuf),
    CopyFile(PathBuf, PathBuf),
}

impl ErrIo {
    pub fn copy_file<P1, P2>(source: P1, dest: P2) -> Self
    where
        P1: Into<PathBuf>,
        P2: Into<PathBuf>,
    {
        Self::CopyFile(source.into(), dest.into())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrNotFound {
    Editor,
    Which,
    UserGroup,
}

impl<T> From<CrossError> for CrossResult<T> {
    fn from(err: CrossError) -> Self {
        Err(err)
    }
}

impl CrossError {
    pub const fn err_io<T>(op: ErrIo, source: std::io::Error) -> CrossResult<T> {
        Err(CrossError::Io { op, source })
    }
    
    pub const fn io(op: ErrIo, source: std::io::Error) -> Self {
        Self::Io { op, source }
    }
    
    pub const fn err_not_found<T>(kind: ErrNotFound) -> CrossResult<T> {
        Err(CrossError::NotFound { kind })
    }
    
    pub const fn not_found(kind: ErrNotFound) -> Self {
        Self::NotFound { kind }
    }
    
    pub const fn err_cmd_call<T>(kind: CmdKind, source: std::io::Error) -> CrossResult<T> {
        Err(CrossError::CmdCall { kind, source })
    }
    
    pub const fn cmd_call(kind: CmdKind, source: std::io::Error) -> Self {
        Self::CmdCall { kind, source }
    }
    
    pub const fn err_cmd<T>(kind: CmdKind, output: std::process::Output) -> CrossResult<T> {
        Err(CrossError::Cmd { kind, output })
    }
    
    pub const fn cmd(kind: CmdKind, output: std::process::Output) -> Self {
        Self::Cmd { kind, output }
    }
    
    pub const fn cmd_checked(kind: CmdKind, check: CmdCheck, output: std::process::Output) -> Self {
        Self::CmdChecked { kind, check, output }
    }
    
    pub fn env_var<S: Into<String>>(var: S, source: std::env::VarError) -> Self {
        Self::EnvVar { var: var.into(), source }
    }
    
    pub fn err_env_var<T, S: Into<String>>(var: S, source: std::env::VarError) -> CrossResult<T> {
        Err(Self::EnvVar { var: var.into(), source })
    }
    
    pub fn err_unsupported<T>() -> CrossResult<T> {
        Err(Self::Unsupported)
    }
    
    pub fn unsupported_dir(kind: CrossDir) -> Self {
        Self::UnsupportedDir { kind }
    }
}