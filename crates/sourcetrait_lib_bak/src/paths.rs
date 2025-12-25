use crate::*;

/// Ergonomic methods for working with paths
pub(crate) trait PathExt {
    fn append_extension(self, ext: &str) -> PathBuf;
    fn filename_string(self) -> Option<String>;
    fn filename_str<'s>(&'s self) -> Option<&'s str>;
}

impl PathExt for PathBuf {
    fn append_extension(self, ext: &str) -> PathBuf {
        if let Some(prev_ext) = self.extension() {
            self.with_extension(format!("{prev_ext}.{ext}", prev_ext = prev_ext.to_str().expect(E_STR)))
        } else {
            self.with_extension(ext)
        }
    }

    fn filename_string(self) -> Option<String> {
        match self.file_name() {
            Some(filename) => match filename.to_str() {
                Some(filename) => Some(filename.to_owned()),
                None => None
            },
            None => None
        }
    }

    fn filename_str<'s>(&'s self) -> Option<&'s str> {
        match self.file_name() {
            Some(filename) => filename.to_str(),
            None => None
        }
    }
}

impl PathExt for &Path {
    fn append_extension(self, ext: &str) -> PathBuf {
        if let Some(prev_ext) = self.extension() {
            self.with_extension(format!("{prev_ext}.{ext}", prev_ext = prev_ext.to_str().expect(E_STR)))
        } else {
            self.with_extension(ext)
        }
    }

    fn filename_string(self) -> Option<String> {
        match self.file_name() {
            Some(filename) => match filename.to_str() {
                Some(filename) => Some(filename.to_owned()),
                None => None
            },
            None => None
        }
    }

    fn filename_str<'s>(&'s self) -> Option<&'s str> {
        match self.file_name() {
            Some(filename) => filename.to_str(),
            None => None
        }
    }
}


fn determine_mirror_dir(base_dir: &Path, src_file: &Path) -> Result<PathBuf, Error> {
    let src_dir = src_file.parent().expect("Expected parent directory");
    let mut mirror_dir = base_dir.to_path_buf();

    for component in src_dir.components() {
        let dirname = component.as_os_str().to_str().expect(E_STR)
            .trim_start_matches("\\\\?\\"); // remove any windows extended path prefix

        match dirname {
            "." | "/" | "\\" => continue,
            ".." => unreachable!("Expected absolute path"),
            _ => {}
        }

        // windows drives (C:, D:, etc)
        if dirname.chars().count() == 2 && dirname.chars().nth(1).unwrap() == ':' {
            mirror_dir.push(dirname.chars().nth(0).unwrap().to_string());
        } else {
            mirror_dir.push(dirname);
        }
    }

    Ok(mirror_dir)
}

pub fn mirror_dir(base_dir: &Path, src_file: &Path, mkdir: bool) -> Result<PathBuf, Error> {
    let mirror_dir = determine_mirror_dir(base_dir, src_file)?;

    if !mirror_dir.is_dir() && mkdir {
        fs::create_dir_all(&mirror_dir)
            .map_err(|e| Error::io(IoOp::Create, &mirror_dir, e))?
    }

    mirror_dir.canonicalize()
        .map_err(|e| Error::io(IoOp::Read, &mirror_dir, e))
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn test_determine_mirror_dir() {
        let base_dir = "/home/dev/.local/share/sourcetrait/bak";
        let src_file = "/home/dev/tmp/source.txt";
        let mirror_dir = determine_mirror_dir(Path::new(base_dir), Path::new(src_file)).unwrap();
        assert_eq!(Path::new("/home/dev/.local/share/sourcetrait/bak/home/dev/tmp"), mirror_dir);
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_determine_mirror_dir_windows() {
        // test a windows path with path extensions
        let base_dir = "\\\\?\\C:\\Users\\dev\\AppData\\Local\\sourcetrait\\bak";
        let src_file = "\\\\?\\C:\\Users\\dev\\tmp\\source.txt";
        let mirror_dir = determine_mirror_dir(Path::new(base_dir), Path::new(src_file)).unwrap();
        assert_eq!("\\\\?\\C:\\Users\\dev\\AppData\\Local\\sourcetrait\\bak\\C\\Users\\dev\\tmp", mirror_dir.to_str().unwrap());
    }
}
