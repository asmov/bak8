mod testlib;

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, process, sync::OnceLock};
    use sourcetrait_testing::{self as testing, prelude::*};
    use sourcetrait_lib_backup as lib_backup;
    use lib_backup::{paths::{self, SourceTraitBackupPath}, sys::GROUP_BACKUP_USERNAME};
    use crate::testlib::GROUP_TESTLIB;

    use super::testlib::{self, TestlibModuleBuilder};

    const BIN_EXE: &str = env!("CARGO_BIN_EXE_sourcetrait_lib_backup");

    static TESTING: testing::Module = testing::module!(Integration, {
            .testlib_module_defaults()
            .using_temp_dir()
    });

    fn exe_sourcetrait_backup<S>(
        test: &testing::Test,
        source_num: u8,
        assert_success: Option<bool>,
        args: &[S]
    ) -> (String, String)
    where
        S: AsRef<std::ffi::OsStr>
    {
        let mock_root = GROUP_TESTLIB.fixture_dir()
            .join(testlib::MOCK_FS_DIRNAME)
            .join(format!("{}{source_num}", testlib::SOURCE_PREFIX));
        let output = process::Command::new(BIN_EXE)
            .args(args)
            .env(paths::consts::ENV_SOURCETRAIT_BACKUP_HOME, mock_root.join(testlib::HOME_TESTUSR))
            .env(testlib::ENV_SOURCETRAIT_BACKUP_TEST_SOURCE_ROOT, &mock_root)
            .env(testlib::ENV_SOURCETRAIT_BACKUP_TEST_TMP_DIR, test.temp_dir())
            .output()
            .unwrap();

        let stdout = strip_ansi_escapes::strip_str(String::from_utf8(output.stdout).unwrap());
        let stderr = strip_ansi_escapes::strip_str(String::from_utf8(output.stderr).unwrap());

        if let Some(success) = assert_success {
            assert_eq!(success, output.status.success(), "Command failed. stdout: {stdout} :: stderr: {stderr}");
        }

        (stdout, stderr)
    }

    /// Returns the (name, dest_dir) of each successful backup.
    fn parse_stdout_backup_results<'stdout>(stdout: &'stdout str) -> Vec<(&'stdout str, PathBuf)> {
        const RE: &str = r"(?m)backup] Completed (?:full|incremental) backup of ([^ ]+) to (.+)$";
        static REGEX: OnceLock<regex::Regex> = OnceLock::new();
        let regex = REGEX.get_or_init(|| regex::Regex::new(RE).unwrap());

        let mut results = Vec::new();
        for (_, [name, dest_dir]) in regex.captures_iter(stdout).map(|c| c.extract()) {
            results.push((name, PathBuf::from(dest_dir).canonicalize().unwrap()));
        }

        results
    }

    #[tested]
    fn test_help() {
        let test = testing::test!({
            .using_temp_dir()
        });

        let (stdout, stderr) = exe_sourcetrait_backup(&test, 1, Some(true), &["backup", "--help"]);
        assert!(stdout.contains("Usage: sourcetrait_lib_backup backup "));
        assert_eq!("", stderr);
    }

    fn setup_backup_dir(test: &mut testing::Test) {
        let config = testlib::make_config(&test, 1);
        SourceTraitBackupPath::StorageDir(test.temp_dir().join(testlib::STRG_SOURCETRAIT_BACKUP)).setup(&config).unwrap();
    }

    #[tested]
    fn test_scheduled() {
        let test = testing::test!({
            .using_temp_dir()
            .setup(setup_backup_dir)
        });

        let (stdout, stderr) = exe_sourcetrait_backup(&test, 1, Some(true), &["backup", "scheduled"]);
        assert_eq!("", stdout);
        assert_eq!("", stderr);
    }

    #[tested]
    fn test_manual_full() {
        let test = testing::test!({
            .using_temp_dir()
            .setup(setup_backup_dir)
        });

        let source_1_dir = testlib::source_dir(1, &test);

        let (stdout, stderr) = exe_sourcetrait_backup(&test, 1, Some(true), &["backup", "full", "home"]);
        assert_eq!("", stderr);
        let results = parse_stdout_backup_results(&stdout);
        assert_eq!(1, results.len());
        assert_eq!("home", results[0].0);
        assert!(!dir_diff::is_different(&source_1_dir, &results[0].1).unwrap());
    }

    #[tested]
    fn test_manual_incremental() {
        let test = testing::test!({
            .using_temp_dir()
            .setup(setup_backup_dir)
        });

        let source_1_dir = testlib::source_dir(1, &test);
        let source_2_dir = testlib::source_dir(2, &test);

        let (stdout, stderr) = exe_sourcetrait_backup(&test, 1, Some(true), &["backup", "full", "home"]);
        assert_eq!("", stderr);
        let results = parse_stdout_backup_results(&stdout);
        assert_eq!(1, results.len());
        assert_eq!("home", results[0].0);
        assert!(!dir_diff::is_different(&source_1_dir, &results[0].1).unwrap());

        let (stdout, stderr) = exe_sourcetrait_backup(&test, 2, Some(true), &["backup", "incremental", "home"]);
        assert_eq!("", stderr);
        let results = parse_stdout_backup_results(&stdout);
        assert_eq!(1, results.len());
        assert_eq!("home", results[0].0);
        assert!(!dir_diff::is_different(&source_2_dir, &results[0].1).unwrap());
    }
}
