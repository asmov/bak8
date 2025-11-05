use crate::*;

pub struct UnsupportedCrossPlatform;
impl CrossPlatform for UnsupportedCrossPlatform {
    fn in_terminal(&self) -> CrossResult<bool> {
        CrossError::err_unsupported()
    }

    fn copy_file<P1, P2>(&self, _source: P1, _dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<std::path::PathBuf>,
        P2: AsRef<Path> + Into<std::path::PathBuf>
    {
            CrossError::err_unsupported()
    }

    fn copy_file_preserved<P1, P2>(&self, _source: P1, _dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<std::path::PathBuf>,
        P2: AsRef<Path> + Into<std::path::PathBuf>
    {
            CrossError::err_unsupported()
    }

    fn primary_user_group(&self) -> CrossResult<String> {
        CrossError::err_unsupported()
    }

    fn run_best_editor<P>(&self, _file: P, _child_process: bool) -> CrossResult<CommandReturn>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        CrossError::err_unsupported()
    }

    fn home_dir(&self) -> CrossResult<PathBuf> {
        CrossError::err_unsupported()
    }

    fn init_dir_for<P>(&self, _base: CrossDir, _subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        CrossError::err_unsupported()
    }

    fn dir_for<P>(&self, _base: CrossDir, _subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        CrossError::err_unsupported()
    }

    fn init_xdg_dir_for<P>(&self, _base: XdgDir, _subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        CrossError::err_unsupported()
    }

    fn xdg_dir_for<P>(&self, _base: XdgDir, _subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        CrossError::err_unsupported()
    }

    fn sanitize_path<P>(&self, _path: P) -> CrossResult<PathBuf>
    where
        P: Into<PathBuf>
    {
        CrossError::err_unsupported()
    }
}
