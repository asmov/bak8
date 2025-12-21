#![allow(dead_code)]

use std::{fs, sync::OnceLock, path::PathBuf};
use chrono::Timelike;
use sourcetrait_testing::{self as testing, prelude::*};
use sourcetrait_crossplat::{self as cross, prelude::*};
use sourcetrait_lib_backup as lib_backup;

pub(crate) const STRG_SOURCETRAIT_BACKUP: &'static str = "strg/sourcetrait/backup";
pub(crate) const TESTLIB: &'static str = "testlib";
pub(crate) const TESTUSR: &'static str = "testusr";
pub(crate) const SOURCE_PREFIX: &'static str = "source-";
pub(crate) const HOME_TESTUSR: &'static str = "home/testusr";
pub(crate) const MOCK_FS_DIRNAME: &'static str = "mock-fs";
/// The test-run's temporary directory
pub(crate) const ENV_SOURCETRAIT_BACKUP_TEST_TMP_DIR: &'static str = "SOURCETRAIT_BACKUP_TEST_TMP_DIR";
/// Where a mock filesystem is located
pub(crate) const ENV_SOURCETRAIT_BACKUP_TEST_SOURCE_ROOT: &'static str = "SOURCETRAIT_BACKUP_TEST_SOURCE_ROOT";

pub(crate) static GROUP_TESTLIB: testing::Group = testing::group!(TESTLIB, Integration, {
    .using_fixture_dir()
});

pub(crate) fn source_dir(source_num: u8, _test: &testing::Test) -> PathBuf {
    GROUP_TESTLIB.fixture_dir()
        .join(MOCK_FS_DIRNAME)
        .join(format!("{}{source_num}", SOURCE_PREFIX))
        .join(HOME_TESTUSR)
        .canonicalize().unwrap()
}

pub(crate) trait TestlibModuleBuilder {
    fn testlib_module_defaults(self) -> Self;
}

impl<'func> TestlibModuleBuilder for testing::ModuleBuilder<'func> {
    fn testlib_module_defaults(self) -> Self {
        lib_backup::run::init(None, None).unwrap();
        //std::env::set_var("BAK8_TEST", "1");//todo
        //call GROUP_TESTLIB instead: self.import_fixture_dir(testlib_namepath());
        self.base_temp_dir(env!("CARGO_TARGET_TMPDIR"))
    }
}

pub(crate) fn sourcetrait_backup_backup(cli: &lib_backup::cli::Cli, config: &lib_backup::config::BackupConfig) -> lib_backup::job::JobResults {
    lib_backup::run::backup::run_backup(&cli, &lib_backup::cli::BackupCommand::Scheduled, Some(config))
}

pub(crate) fn setup_backup_dir(test: &testing::Test, config: &lib_backup::config::BackupConfig) {
    lib_backup::SourceTraitBackupPath::StorageDir(test.temp_dir().join(STRG_SOURCETRAIT_BACKUP)).setup(config).unwrap();
}

pub(crate) fn make_scheduled_backup_cli(_test: &testing::Test) -> lib_backup::cli::Cli {
    lib_backup::cli::Cli {
        config_file: None,
        force: false,
        quiet: false,
        subcommand: lib_backup::cli::Command::Backup(lib_backup::cli::BackupCommand::Scheduled),
    }
}

pub(crate) fn make_config(test: &testing::Test, source_version: u8) -> lib_backup::config::BackupConfig {
    let user = cross::PLATFORM.access().current_user().unwrap();
    let username = user.username().try_into_utf8().unwrap();
    let usergroup = cross::PLATFORM.access().user_primary_group(&user).unwrap().expect("group").groupname().try_into_utf8().unwrap();
    lib_backup::config::BackupConfig {
        backup_storage_dir: test.temp_dir().join(STRG_SOURCETRAIT_BACKUP)
            .to_str().unwrap().to_string(),
        storage_admin_user: username.to_string(),
        backup_users_group: usergroup.to_string(),
        schedules: vec![
            lib_backup::config::BackupConfigSchedule {
                name: "minutely".to_string(),
                minute: None,
                minutes: Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
                    51, 52, 53, 54, 55, 56, 57, 58, 59]),
                hour: None,
                hours: None,
                day_of_week: None,
                days_of_week: None,
                day_of_month: None,
                days_of_month: None,
                month: None,
                months: None,
            },
        ],
        remotes: vec![],
        remote_groups: vec![],
        backups: vec![
            lib_backup::config::BackupConfigBackup {
                name: "home".to_string(),
                source_dir: GROUP_TESTLIB.fixture_dir()
                    .join(MOCK_FS_DIRNAME)
                    .join(format!("{}{source_version}", SOURCE_PREFIX))
                    .join(HOME_TESTUSR)
                    .to_str().unwrap().to_string(),
                full_schedule: "daily".to_string(),
                incremental_schedule: "minutely".to_string(),
                max_full: 4,
                archives: vec![],
                syncs: vec![],
            },
        ],
    }
}

/// tests are configured to run full backups hourly. if those tests run at the top of the hour, they may fail.
pub(crate) fn sleep_if_top_of_hour() {
    if chrono::Local::now().minute() > 58 {
        println!("[debug] Top of the hour. Sleeping for 60 seconds to ensure tests operate normally...");
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}


pub(crate) fn setup_incremental_backup_test(test: &mut testing::Test) {
    let hostname = cross::PLATFORM.net().hostname().unwrap();
    sleep_if_top_of_hour();

    let config = make_config(test, 1);
    setup_backup_dir(test, &config);

    let cli = make_scheduled_backup_cli(test);
    let mut results = sourcetrait_backup_backup(&cli, &config).unwrap();
    assert_eq!(2, results.len());
    results.pop().unwrap();
    let backup_output = match results.pop().unwrap() {
        lib_backup::JobOutput::Backup(job) => job, _ => panic!() };

    let earlier_run_name = lib_backup::backup::BackupRunName::new(
        chrono::Local::now().checked_sub_signed(chrono::Duration::minutes(1)).unwrap(),
        &hostname,
        &hostname,
        &config.backups[0].name,
    );

    let earlier_backup_run_dir = lib_backup::paths::SourceTraitBackupPath::backup(
        test.temp_dir().join(STRG_SOURCETRAIT_BACKUP),
        lib_backup::backup::BackupType::Full,
        &earlier_run_name);

    fs::rename(&backup_output.dest_dir, earlier_backup_run_dir).unwrap();
}

/// Remote testing need either "sshd.test" or localhost to be configured for SSH pubkey login (~/.ssh/config)
/// We will try to resolve "sshd.test" first and then fallback to localhost
pub(crate) fn resolve_test_remote() -> &'static lib_backup::Remote {
    static TEST_REMOTE: OnceLock<lib_backup::Remote> = OnceLock::new();
    TEST_REMOTE.get_or_init(|| {
        let host = match std::net::TcpStream::connect("sshd.test:22") {
            Ok(_) => "sshd.test".to_string(),
            Err(_) => {
                eprintln!("WARNING: Could not connect to 'sshd.test:22'. Falling back to 'localhost'");
                "localhost".to_string()
            }
        };

        lib_backup::Remote {
            name: "sshd-test".to_string(),
            host: host.clone(),
            user: None,
            platform: lib_backup::Platform::GNU,
        }
    })
}

/// Creates the necessary backup directories on the remote
pub(crate) fn setup_test_remote(config: &lib_backup::config::BackupConfig) -> lib_backup::config::BackupConfigRemote {
    let remote = resolve_test_remote();
    let temp_dir = lib_backup::cmd::ssh_run::ssh_temp_dir(&remote).unwrap();
    let storage_dir = lib_backup::SourceTraitBackupPath::RemoteStorageDir { remote: remote.clone(), storage_dir: temp_dir.join(STRG_SOURCETRAIT_BACKUP) };

    storage_dir.setup(config).unwrap();

    lib_backup::config::BackupConfigRemote {
        name: remote.name.clone(),
        host: remote.host.clone(),
        user: remote.user.clone(),
        platform: remote.platform.clone(),
        backup_storage_dir: storage_dir.as_path().to_str().unwrap().to_string(),
    }
}

pub(crate) fn teardown_test_remote(cfg_remote: &lib_backup::config::BackupConfigRemote) {
    let dir = PathBuf::from(&cfg_remote.backup_storage_dir)
        .parent().unwrap()
        .parent().unwrap().to_path_buf();

    // prevent disaster on test system
    let parent_dir = dir.parent().unwrap();
    assert_eq!(parent_dir, PathBuf::from("/tmp"), "DANGER: Attempting to delete non-temporary directory!");

    lib_backup::cmd::ssh_run::ssh_delete_dir(
        &cfg_remote.into(),
        &dir
    ).unwrap();
}

pub(crate) fn make_sync_config(
    test: &testing::Test,
    cfg_remote: &lib_backup::config::BackupConfigRemote,
    source_version: u8
) -> lib_backup::config::BackupConfig {
    let mut config = make_config(test, source_version);
    config.remotes = vec![ cfg_remote.clone() ];

    config.backups[0].syncs = vec![
        lib_backup::config::BackupConfigSync {
            remote: Some("sshd-test".to_string()),
            remote_group: None,
            sync_full: true,
            sync_incremental: true,
            sync_archive: true
        }
    ];

    config
}

pub(crate) fn expected_backup_ouput_dir(test: &testing::Test, backup_type: lib_backup::BackupType, output: &lib_backup::BackupJobOutput) -> PathBuf {
    let hostname = cross::PLATFORM.net().hostname().unwrap();
    test.temp_dir()
        .join(STRG_SOURCETRAIT_BACKUP)
        .join(backup_type.subdir_name())
        .join(&*hostname)
        .join(&*hostname)
        .join(output.run_name.datetime.format("%Y").to_string())
        .join(output.run_name.datetime.format("%m").to_string())
        .join(output.run_name.datetime.format("%d").to_string())
        .join(output.run_name.to_string())
}

pub(crate) fn assert_remote_backup_synced(
    _test: &testing::Test,
    backup_type: lib_backup::BackupType,
    source_num: u8,
    remote_cfg: &lib_backup::config::BackupConfigRemote,
    output: &lib_backup::SyncBackupJobOutput
) {
    let hostname = cross::PLATFORM.net().hostname().unwrap();
    assert_eq!(remote_cfg.name, output.remote.name);

    let expected_dir = PathBuf::from(&remote_cfg.backup_storage_dir)
        .join(backup_type.subdir_name())
        .join(&*hostname)
        .join(&*hostname)
        .join(output.backup_run_name.datetime.format("%Y").to_string())
        .join(output.backup_run_name.datetime.format("%m").to_string())
        .join(output.backup_run_name.datetime.format("%d").to_string())
        .join(output.backup_run_name.to_string());
    assert_eq!(expected_dir, output.remote_dest_dir.as_path());

    // remotely test to see if the expected dir exists
    assert!(lib_backup::cmd::ssh_run::ssh_dirs_exist(&output.remote, &vec![&expected_dir]).unwrap());

    // compare sha256sum manifest between testing source and remote destination
    let remote_manifest = lib_backup::cmd::ssh_run::ssh_dir_manifest(&output.remote, &expected_dir).unwrap();
    let expected_manifest = fs::read_to_string(GROUP_TESTLIB.fixture_dir()
        .join(MOCK_FS_DIRNAME)
        .join(format!("{}{source_num}", SOURCE_PREFIX))
        .with_extension("manifest")).unwrap().trim().to_string();
    assert_eq!(expected_manifest, remote_manifest);
}

pub(crate) fn assert_remote_archive_synced(
    _test: &testing::Test,
    remote_cfg: &lib_backup::config::BackupConfigRemote,
    archive_output: &lib_backup::ArchiveJobOutput,
    output: &lib_backup::SyncArchiveJobOutput
) {
    let hostname = cross::PLATFORM.net().hostname().unwrap();
    assert_eq!(remote_cfg.name, output.remote.name);

    let expected_filepath = PathBuf::from(&remote_cfg.backup_storage_dir)
        .join(lib_backup::paths::consts::BACKUP_ARCHIVE_DIRNAME)
        .join(&*hostname)
        .join(&*hostname)
        .join(output.backup_run_name.datetime.format("%Y").to_string())
        .join(output.backup_run_name.datetime.format("%m").to_string())
        .join(output.backup_run_name.datetime.format("%d").to_string())
        .join(output.backup_run_name.to_string())
        .with_extension(lib_backup::paths::consts::TAR_XZ_EXTENSION);
    assert_eq!(expected_filepath, output.remote_dest_filepath.as_path());

    let expected_checksum_filepath = PathBuf::from(&remote_cfg.backup_storage_dir)
        .join(lib_backup::paths::consts::BACKUP_ARCHIVE_DIRNAME)
        .join(&*hostname)
        .join(&*hostname)
        .join(output.backup_run_name.datetime.format("%Y").to_string())
        .join(output.backup_run_name.datetime.format("%m").to_string())
        .join(output.backup_run_name.datetime.format("%d").to_string())
        .join(output.backup_run_name.to_string())
        .with_extension(format!("{}.{}", lib_backup::paths::consts::TAR_XZ_EXTENSION, lib_backup::paths::consts::SHA256_EXTENSION));
    assert_eq!(expected_checksum_filepath, output.remote_dest_checksum_filepath.as_path());

    // remotely test to see if the expected dir exists
    assert!(lib_backup::cmd::ssh_run::ssh_files_exist(&output.remote, &vec![
        &expected_filepath,
        &expected_checksum_filepath]).unwrap());

    // compare sha256sum manifest between testing source and remote destination
    let remote_checksum = lib_backup::cmd::ssh_run::ssh_file_contents(&output.remote, &expected_checksum_filepath)
        .unwrap().unwrap()
        .split_ascii_whitespace()
        .next().unwrap().to_string();
    assert_eq!(archive_output.checksum, remote_checksum);
}
