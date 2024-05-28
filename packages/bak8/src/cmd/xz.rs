use std::path::Path;

/// Equivalent to:
/// ```bash
/// cd "$source_dir/.." &&
/// tar cf "$zip_file" --use-compress-program='xz -T0' "$(basename "$source_dir")"
/// ```
pub fn cmd_tar_xz(source_dir: &Path, zip_file: &Path) -> std::process::Command {
    let mut cmd = std::process::Command::new("tar");
    cmd
        .current_dir(source_dir.parent().unwrap())
        .args(&[
            "cf",
            zip_file.to_str().unwrap(),
            "--use-compress-program",
            "xz -T0",
            source_dir.file_name().unwrap().to_str().unwrap(),
        ]);

    cmd
}

