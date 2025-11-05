use crate::*;

pub trait CrossPlatform {
    fn in_terminal(&self) -> CrossResult<bool>;
    
    fn copy_file<P1, P2>(&self, source: P1, dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<PathBuf>,
        P2: AsRef<Path> + Into<PathBuf>;
    
    fn copy_file_preserved<P1, P2>(&self, source: P1, dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<PathBuf>,
        P2: AsRef<Path> + Into<PathBuf>;
    
    fn primary_user_group(&self) -> CrossResult<String>;
    
    fn run_best_editor<P>(&self, file: P, child_process: bool) -> CrossResult<CommandReturn>
    where
        P: AsRef<Path> + Into<PathBuf>;
    
    fn home_dir(&self) -> CrossResult<PathBuf>;
    
    fn init_dir_for<P>(&self, base: CrossDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>;
    
    fn dir_for<P>(&self, base: CrossDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>;
    
    fn init_xdg_dir_for<P>(&self, base: XdgDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>;
    
    fn xdg_dir_for<P>(&self, base: XdgDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>;
    
    fn sanitize_path<P>(&self, path: P) -> CrossResult<PathBuf>
    where
        P: Into<PathBuf>;
}

pub struct Platform<OS: CrossPlatform> {
    os: OS
}

impl<OS: CrossPlatform> Platform<OS> {
    pub const fn new(os: OS) -> Self {
        Self {
            os
        }
    }
}

impl<OS: CrossPlatform> CrossPlatform for Platform<OS> {
    #[inline]
    fn in_terminal(&self) -> CrossResult<bool> {
        self.os.in_terminal()
    }

    #[inline]
    fn copy_file<P1, P2>(&self, source: P1, dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<PathBuf>,
        P2: AsRef<Path> + Into<PathBuf>,
    {
        self.os.copy_file(source, dest)
    }
    
    #[inline]
    fn copy_file_preserved<P1, P2>(&self, source: P1, dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<PathBuf>,
        P2: AsRef<Path> + Into<PathBuf>,
    {
        self.os.copy_file_preserved(source, dest)
    }

    #[inline]
    fn primary_user_group(&self) -> CrossResult<String> {
        self.os.primary_user_group()
    }
    
    #[inline]
    fn run_best_editor<P>(&self, file: P, child_process: bool) -> CrossResult<CommandReturn>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        self.os.run_best_editor(file, child_process)
    }

    #[inline]
    fn home_dir(&self) -> CrossResult<PathBuf> {
        self.os.home_dir()
    }

    #[inline]
    fn init_dir_for<P>(&self, base: CrossDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        self.os.init_dir_for(base, subdir)
    }
    
    #[inline]
    fn dir_for<P>(&self, base: CrossDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        self.os.dir_for(base, subdir)
    }

    #[inline]
    fn init_xdg_dir_for<P>(&self, base: XdgDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        self.os.init_xdg_dir_for(base, subdir)
    }

    #[inline]
    fn xdg_dir_for<P>(&self, xdg_dir: XdgDir, subdir: P) -> CrossResult<PathBuf>
    where
        P: AsRef<Path> + Into<PathBuf>
    {
        self.os.xdg_dir_for(xdg_dir, subdir)
    }

    #[inline]
    fn sanitize_path<P>(&self, path: P) -> CrossResult<PathBuf>
    where
        P: Into<PathBuf>
    {
        self.os.sanitize_path(path)
    }
}

