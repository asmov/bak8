use crate::*;

#[cfg(target_os = "linux")]
pub const PLATFORM: Platform<linux::LinuxCrossPlatform> = Platform::new(linux::LinuxCrossPlatform);
#[cfg(target_os = "macos")]
pub const PLATFORM: Platform<macos::MacOsCrossPlatform> = Platform::new(macos::MacOsCrossPlatform);
#[cfg(target_os = "windows")]
pub const PLATFORM: Platform<windows::LinuxCrossPlatform> = Platform::new(windows::LinuxCrossPlatform);
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub const PLATFORM: Platform<unsupported::UnsupportedCrossPlatform> = Platform::new(windows::UnsupportedCrossPlatform);
