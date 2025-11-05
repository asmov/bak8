use crate::*;

pub struct WindowsCrossPlatform;
impl CrossPlatform for WindowsCrossPlatform {
    fn in_terminal(&self) -> CrossResult<bool> {
        todo!("Windows is awaiting support")
    }

    fn copy_file<P1, P2>(&self, src: P1, dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<std::path::PathBuf>,
        P2: AsRef<Path> + Into<std::path::PathBuf>
    {
        std_copy_file(src, dest)
    }

    fn copy_file_preserved<P1, P2>(&self, _source: P1, _dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<std::path::PathBuf>,
        P2: AsRef<Path> + Into<std::path::PathBuf>
    {
        todo!("Windows is awaiting support")
    }

    fn primary_user_group(&self) -> CrossResult<String> {
        primary_user_group()
    }

    fn run_best_editor<P>(&self, _file: P, _child_process: bool) -> CrossResult<CommandReturn>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        todo!("Windows is awaiting support")
    }
    
    fn sanitize_path<P>(&self, path: P) -> CrossResult<PathBuf>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        Ok(path.to_string_lossy().trim_start_matches("\\\\?\\").into())
    }

    fn home_dir(&self) -> CrossResult<PathBuf> {
        home_dir()
    }

    fn init_dir_for<P>(&self, base: CrossDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        init_dir_for(base, subdir)
    }

    fn dir_for<P>(&self, base: CrossDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        dir_for(base, subdir)
    }

    fn init_xdg_dir_for<P>(&self, base: XdgDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        init_xdg_dir_for(base, subdir)
    }

    fn xdg_dir_for<P>(&self, base: XdgDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        xdg_dir_for(base, subdir)
    }
}
