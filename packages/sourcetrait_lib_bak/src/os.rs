use std::path::Path;
use crate::*;

use crate::E_STR;

pub fn print_diff(source: &Path, file_b: &Path) -> Result<(), crate::Error> {
    if !crate::diff_files(source, file_b)? {
        println!("No difference");
        return Ok(());
    }

    // try `git diff` first. if not available, use a system-specific diff command
    let output = std::process::Command::new("git")
        .arg("diff")
        .arg("--no-index")
        .arg("--color")
        .arg(file_b)
        .arg(source)
        .output();

    match output {
        Ok(output) => {
            let lines: String = String::from_utf8(output.stdout).expect(E_STR).trim()
                .lines()
                .skip(2)
                .map(|line| format!("{line}\n"))
                .collect();

            println!("{}", lines.trim());
            return Ok(());
        },
        Err(_) => {} // try system 'diff'
    }

    if cfg!(any(target_os = "linux", target_os = "macos")) {
        let output = std::process::Command::new("diff")
            .arg("--color=always")
            .arg("-c")
            .arg(file_b)
            .arg(source)
            .output()
            .map_err(|e| crate::Error::msg(e.to_string()))?;

        println!("{}", String::from_utf8(output.stdout).expect(E_STR).trim());
        Ok(())
    } else if cfg!(target_os = "windows") {
        let output = std::process::Command::new("powershell")
            .arg("compare-object")
            .arg(format!("(get-content {})", file_b.to_string_lossy()))
            .arg(format!("(get-content {})", source.to_string_lossy()))
            .output()
            .map_err(|e| crate::Error::msg(e.to_string()))?;

        println!("{}", String::from_utf8(output.stdout).expect(E_STR).trim());
        Ok(())
    } else {
        Err(crate::Error::msg("Unsupported OS".to_string()))
    }
}

pub(crate) fn copy_file_preserved<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dst: P2) -> Result<(), io::Error> {
    #[cfg(target_os = "windows")] {
        fs::copy(&src, &dst)
    }
    #[cfg(target_os = "linux")] {
        fs::copy(&src, &dst)
    }
    #[cfg(target_os = "macos")] {
        self::macos::copy_file_preserved(src, dst)
    }
}

#[cfg(target_os = "linux")]
pub(crate) mod linux {
    use crate::*;
    use std::{
        os::unix::{
            ffi::OsStrExt,
            fs::MetadataExt,
        },
        ffi::CString,
    };
    
    pub(super) fn copy_file_preserved<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dst: P2) -> Result<(), io::Error> {
        std::fs::copy(src, dst)?;
        
        let metadata = src.metadata()?;
        let src_c = CString::new(src.as_os_str().as_bytes())?;
        let dst_c = CString::new(dst.as_os_str().as_bytes())?;
        
        unsafe {
            libc::chown(
                dst_c.as_ptr(),
                metadata.uid(),
                metadata.gid(),
            );
            
            libc::chmod(dst_c.as_ptr(), metadata.mode());
            
            let times = [
                libc::timespec {
                    tv_sec: metadata.atime(),
                    tv_nsec: metadata.atime_nsec(),
                },
                libc::timespec {
                    tv_sec: metadata.mtime(),
                    tv_nsec: metadata.mtime_nsec(),
                },
            ];
            libc::utimensat(
                libc::AT_FDCWD,
                dst_c.as_ptr(),
                times.as_ptr(),
                0,
            );
        }
        
        copy_xattrs(&src_c, &dst_c)?;
        copy_selinux_context(&src_c, &dst_c)?;
        Ok(())
    }
    
    fn copy_xattrs(src: &CStr, dst: &CStr) -> std::io::Result<()> {
        unsafe {
            let size = libc::listxattr(
                src.as_ptr(),
                std::ptr::null_mut(),
                0,
            );
            
            if size <= 0 {
                return Ok(());
            }
            
            let mut list = vec![0u8; size as usize];
            libc::listxattr(
                src.as_ptr(),
                list.as_mut_ptr() as *mut i8,
                size as usize,
            );
            
            let mut pos = 0;
            while pos < list.len() {
                let name_start = pos;
                while pos < list.len() && list[pos] != 0 {
                    pos += 1;
                }
                
                if pos <= name_start {
                    break;
                }
                
                let name = CStr::from_bytes_with_nul(&list[name_start..=pos]).unwrap();
                
                let value_size = libc::getxattr(
                    src.as_ptr(),
                    name.as_ptr(),
                    std::ptr::null_mut(),
                    0,
                );
                
                if value_size > 0 {
                    let mut value = vec![0u8; value_size as usize];
                    libc::getxattr(
                        src.as_ptr(),
                        name.as_ptr(),
                        value.as_mut_ptr() as *mut libc::c_void,
                        value_size as usize,
                    );
                    
                    libc::setxattr(
                        dst.as_ptr(),
                        name.as_ptr(),
                        value.as_ptr() as *const libc::c_void,
                        value.len(),
                        0,
                    );
                }
                
                pos += 1;
            }
        }
        
        Ok(())
    }
    
    fn copy_selinux_context(src: &CStr, dst: &CStr) -> std::io::Result<()> {
        unsafe {
            let context_name = CString::new("security.selinux").unwrap();
            let size = libc::getxattr(
                src.as_ptr(),
                context_name.as_ptr(),
                std::ptr::null_mut(),
                0,
            );
            
            if size > 0 {
                let mut context = vec![0u8; size as usize];
                libc::getxattr(
                    src.as_ptr(),
                    context_name.as_ptr(),
                    context.as_mut_ptr() as *mut libc::c_void,
                    size as usize,
                );
                
                libc::setxattr(
                    dst.as_ptr(),
                    context_name.as_ptr(),
                    context.as_ptr() as *const libc::c_void,
                    context.len(),
                    0,
                );
            }
        }
        
        Ok(())
    }
}

#[cfg(target_os = "macos")]
pub(crate) mod macos {
    use crate::*;
    use std::{
        ffi::CString,
        os::unix::ffi::OsStrExt,
    };
    
    unsafe extern "C" {
        fn clonefile(src: *const libc::c_char, dst: *const libc::c_char, flags: u32) -> libc::c_int;
    }
    
    pub(super) fn copy_file_preserved<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dst: P2) -> Result<(), io::Error> {
        let src = CString::new(src.as_ref().as_os_str().as_bytes())?;
        let dst = CString::new(dst.as_ref().as_os_str().as_bytes())?;
        
        let result = unsafe {
            clonefile(src.as_ptr(), dst.as_ptr(), 0)
        };
        
        match result {
            0 => Ok(()),
            _ => Err(std::io::Error::last_os_error())
        }
    }
}