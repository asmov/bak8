use crate::*;
use crate::macos::*;

pub struct MacOsCrossPlatform;
impl CrossPlatform for MacOsCrossPlatform {
    fn in_terminal(&self) -> CrossResult<bool> {
        match env::var(ENV_TERM_SESSION_ID) {
            Ok(_) => Ok(true),
            Err(env::VarError::NotPresent) => Ok(false),
            Err(source) => CrossError::err_env_var(ENV_VAR_TERM_SESSION_ID, source),
        }
    }
    
    fn copy_file<P1, P2>(&self, source: P1, dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<PathBuf>,
        P2: AsRef<Path> + Into<PathBuf>,
    {
        std_copy_file(source, dest)
    }
    
    fn copy_file_preserved<P1, P2>(&self, source: P1, dest: P2) -> CrossResult<()>
    where
        P1: AsRef<Path> + Into<PathBuf>,
        P2: AsRef<Path> + Into<PathBuf>,
    {
        exec_cp_preserved(source, dest)
    }
    
    fn primary_user_group(&self) -> CrossResult<String> {
        primary_user_group()
    }

    fn run_best_editor<P>(&self, file: P, child_process: bool) -> CrossResult<CommandReturn>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        let sys_open_cmd = match self.in_terminal()? {
            true => None,
            false => Some(cmd_open(file.as_ref()))
        };
        
        unixy::run_best_editor(sys_open_cmd, file, child_process)
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
    
    fn sanitize_path<P>(&self, path: P) -> CrossResult<PathBuf>
    where
        P: Into<PathBuf>,
    {
        Ok(path.into())
    }
}

fn cmd_open<P>(file: P) -> process::Command
where
    P: AsRef<Path> + Into<PathBuf>,
{
    let mut cmd = Command::new(CMD_OPEN);
    cmd.arg(file.as_ref());
    cmd
}

fn exec_cp_preserved<P1, P2>(source: P1, dest: P2) -> CrossResult<()>
where
    P1: AsRef<Path> + Into<PathBuf>,
    P2: AsRef<Path> + Into<PathBuf>,
{
    const CP_ARG_ARCHIVE: &'static str = "-a";
    
    let mut cmd = Command::new(unixy::CMD_CP);
    cmd
        .arg(CP_ARG_ARCHIVE)
        .arg(source.as_ref())
        .arg(dest.as_ref());
    
    CommandReturn::from(CmdKind::Copy, cmd)?;
    Ok(())
}

