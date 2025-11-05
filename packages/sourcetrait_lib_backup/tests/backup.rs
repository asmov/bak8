mod testlib;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use sourcetrait_testing::{self as testing, prelude::*};
    use sourcetrait_lib_backup as lib_backup;
    use lib_backup::log::TikPath;
    use lib_backup::job::*;
    use super::testlib::{self, TestlibModuleBuilder};

    static TESTING: testing::Module = testing::module!(Integration, {
        .testlib_module_defaults()
        .using_temp_dir()
    });

    #[tested]
    fn test_scheduled_full() {
        let test = testing::test!({
            .using_temp_dir()
        });

        let config = testlib::make_config(&test, 1);
        testlib::setup_backup_dir(&test, &config);

        let cli = testlib::make_scheduled_backup_cli(&test);

        let mut results = testlib::bak8_backup(&cli, &config).unwrap();
        assert_eq!(2, results.len());
        let JobOutput::Archive(archive_output) = results.pop().unwrap() else { panic!() };
        let JobOutput::Backup(backup_output) = results.pop().unwrap() else { panic!() };

        assert_eq!(testlib::expected_backup_ouput_dir(&test, lib_backup::BackupType::Full, &backup_output),
            backup_output.dest_dir.as_path());

        assert_eq!(false, dir_diff::is_different(
            &backup_output.source_dir,
            &backup_output.dest_dir.as_path()).unwrap());
        assert_eq!(true, archive_output.dest_filepath.as_path().exists());

        // try running it again. it should not create a new backup for "today"
        let results = testlib::bak8_backup(&cli, &config).unwrap();
        assert_eq!(0, results.len());
    }

    #[tested]
    fn test_scheduled_incremental() {
        let test = testing::test!({
            .using_temp_dir()
            .setup(testlib::setup_incremental_backup_test)
        });

        // full backup was ran during test.setup(). now run an incremental ...
        let cli = testlib::make_scheduled_backup_cli(&test);
        let config = testlib::make_config(&test, 2);
        let mut results = testlib::bak8_backup(&cli, &config).unwrap();

        assert_eq!(1, results.len(), "{:#?}", results);
        let JobOutput::Backup(backup_output) = results.pop().unwrap() else { panic!() };

        assert_eq!(testlib::expected_backup_ouput_dir(&test, lib_backup::BackupType::Incremental,
            &backup_output), backup_output.dest_dir.as_path());

        let dest_dir = backup_output.dest_dir.as_path();
        assert!(!dir_diff::is_different(&backup_output.source_dir, &dest_dir).unwrap(),
            "Incremental source and destination should not be different: {} VS {}",
            &backup_output.source_dir.tikn_path(), &dest_dir.tikn_path());
        assert!(dest_dir.join("source-2.txt").exists(),
            "New file: source-2.txt");
        assert_eq!("source-2 delta", fs::read_to_string(dest_dir.join("delta.txt")).unwrap(),
            "Modified file: delta.txt");
        
        let source_perms = fs::metadata(backup_output.source_dir.join("alpha").join("alpha.txt"))
            .unwrap().permissions().mode() & 0o777;
        
        assert_eq!(source_perms, fs::metadata(dest_dir.join("alpha").join("alpha.txt")).unwrap().permissions().mode() & 0o777,
            "Modified permissions: alpha/alpha.txt");

        // try running it again. it should not create a new backup for "today"
        let results = testlib::bak8_backup(&cli, &config).unwrap();
        assert_eq!(0, results.len());
    }
}
