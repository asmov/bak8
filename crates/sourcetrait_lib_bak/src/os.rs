use crate::*;

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
