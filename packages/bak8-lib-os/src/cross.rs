use std::path::Path;
use super::CommandReturn;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
mod unsupported;



pub trait CrossPlatform {
    fn in_terminal(&self) -> bool;
    fn run_best_editor(&self, file: &Path, child_process: bool) -> anyhow::Result<CommandReturn>;
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
    fn in_terminal(&self) -> bool {
        self.os.in_terminal()
    }

    #[inline]
    fn run_best_editor(&self, file: &Path, child_process: bool) -> anyhow::Result<CommandReturn> {
        self.os.run_best_editor(file, child_process)
    }
}

#[cfg(target_os = "linux")]
pub const PLATFORM: Platform<linux::LinuxCrossPlatform> = Platform::new(linux::LinuxCrossPlatform);
#[cfg(target_os = "macos")]
pub const PLATFORM: Platform<macos::LinuxCrossPlatform> = Platform::new(macos::LinuxCrossPlatform);
#[cfg(target_os = "windows")]
pub const PLATFORM: Platform<windows::LinuxCrossPlatform> = Platform::new(windows::LinuxCrossPlatform);
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub const PLATFORM: Platform<windows::LinuxCrossPlatform> = Platform::new(windows::LinuxCrossPlatform);
