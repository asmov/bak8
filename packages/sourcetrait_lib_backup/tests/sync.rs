mod testlib;

#[cfg(test)]
mod tests {
    use super::testlib::{self, TestlibModuleBuilder};
    use sourcetrait_testing::{self as testing, prelude::*};
    use sourcetrait_lib_backup as lib_backup;

    static TESTING: testing::Module = testing::module!(Integration, {
            .testlib_module_defaults()
            .using_temp_dir()
    });

    fn setup_sync_test(
        test: &testing::Test,
        config: &lib_backup::config::BackupConfig,
        _cfg_remote: &lib_backup::config::BackupConfigRemote,
    ) {
        testlib::setup_backup_dir(test, config);
    }

    //fn teardown_sync_test(_test: &mut testing::Test, cfg_remote: &lib_backup::config::BackupConfigRemote) {
    //    testlib::teardown_test_remote(cfg_remote);
    //}

    #[tested]
    #[ignore = "Requires an SSH server to be set up as sshd_test. Run manually."]
    fn test_full_sync() {
        let test = testing::test!({
            .using_temp_dir()
            //.teardown(|test| teardown_sync_test(test, &cfg_remote))
        });

        let config = testlib::make_config(&test, 1);
        let cfg_remote = testlib::setup_test_remote(&config);
        setup_sync_test(&test, &config, &cfg_remote);

        let cli = testlib::make_scheduled_backup_cli(&test);
        let config = testlib::make_sync_config(&test, &cfg_remote, 1);
        let mut results = testlib::bak8_backup(&cli, &config).unwrap();

        assert_eq!(4, results.len(), "{:#?}", results);
        let lib_backup::JobOutput::SyncArchive(sync_archive_output) = results.pop().unwrap() else {
            panic!("not SynArchive")
        };
        let lib_backup::JobOutput::SyncBackup(sync_backup_output) = results.pop().unwrap() else {
            panic!("not SyncBackup")
        };
        let lib_backup::JobOutput::Archive(archive_output) = results.pop().unwrap() else {
            panic!("not Archive")
        };
        let lib_backup::JobOutput::Backup(_backup_output) = results.pop().unwrap() else {
            panic!("not Backup")
        };

        /* todo: verify file permissions:
            installed:
            - backup storage dir: 0750 user:user or root:bak8usr
            - backup storage subdirs: 0750 user:user or root:bak8user
            - backup storage subdirs localhost dir: 0750 user:user or root:bak8usr
              - full, incremental, archive, logs
            prepared (context):
            - full backup host/user dir: 0700 user:user
            - incremental backup host/user dir: 0700 user:user
            - archive host/user dir: 0700 user:user
            - backup host/user/run dir: 0700 user:user
            - archive host/user/run files: 0600 user:user
        */

        testlib::assert_remote_backup_synced(
            &test,
            lib_backup::BackupType::Full,
            1,
            &cfg_remote,
            &sync_backup_output,
        );
        testlib::assert_remote_archive_synced(
            &test,
            &cfg_remote,
            &archive_output,
            &sync_archive_output,
        );
    }
}
