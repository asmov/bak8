use crate::*;

#[derive(Debug, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum IoOp {
    Read,
    Write,
    Delete,
    Rename,
    Create
}

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("Unable to {op} {path}: {cause}"))]
    IO { op: IoOp, path: String, cause: String },

    #[snafu(display("Unable to copy {src} to {dest}: {cause}"))]
    Copy { src: String, dest: String, cause: String },

    #[snafu(display("Invalid index for {src}: {index}"))]
    Index { src: String, index: u8 },
    
    #[snafu(display("{source}"))]
    Cross { source: cross::CrossError },

    #[snafu(display("{msg}"))]
    Generic { msg: String }
}

impl Error {
    pub fn io(op: IoOp, path: &Path, cause: std::io::Error) -> Self {
        Self::IO { op, path: path.to_str().expect(E_STR).cyan().to_string(), cause: cause.to_string() }
    }

    pub fn index(source: &Path, index: u8) -> Self {
        Self::Index { src: source.to_str().expect(E_STR).cyan().to_string(), index }
    }

    pub fn copy(source: &Path, destination: &Path, cause: cross::CrossError) -> Self {
        Self::Copy {
            src: source.to_str().expect(E_STR).cyan().to_string(),
            dest: destination.to_str().expect(E_STR).cyan().to_string(),
            cause: cause.to_string() }
    }
    
    pub(crate) fn msg<S: std::fmt::Display>(msg: S) -> Self {
        Self::Generic { msg: msg.to_string() }
    }
}

impl From<cross::CrossError> for Error {
    fn from(source: cross::CrossError) -> Self {
        Self::Cross { source }
    }
}