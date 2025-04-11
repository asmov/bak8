#![allow(dead_code)]

use std::{fs, sync::OnceLock, path::PathBuf};
use chrono::Timelike;
use asmov_common_testing::{self as testing, prelude::*};
use bak8;
use whoami;

pub(crate) const STRG_BAK8: &'static str = "strg/bak8";
pub(crate) const TESTLIB: &'static str = "testlib";
pub(crate) const TESTUSR: &'static str = "testusr";
pub(crate) const SOURCE_PREFIX: &'static str = "source-";
pub(crate) const HOME_TESTUSR: &'static str = "home/testusr";
pub(crate) const MOCK_FS_DIRNAME: &'static str = "mock-fs";
/// The test-run's temporary directory
pub(crate) const ENV_BAK8_TEST_TMP_DIR: &'static str = "BAK8_TEST_TMP_DIR";
/// Where a mock filesystem is located
pub(crate) const ENV_BAK8_TEST_SOURCE_ROOT: &'static str = "BAK8_TEST_SOURCE_ROOT";

pub(crate) fn source_dir(source_num: u8, test: &testing::Test) -> PathBuf {
    test.imported_fixture_dir(testlib_namepath())
        .join(MOCK_FS_DIRNAME)
        .join(format!("{}{source_num}", SOURCE_PREFIX))
        .join(HOME_TESTUSR)
        .canonicalize().unwrap()
}

pub(crate) fn testlib_namepath() -> &'static testing::Namepath {
    static NAMEPATH: OnceLock<testing::Namepath> = OnceLock::new();
    &NAMEPATH.get_or_init(|| testing::Namepath::module(testing::UseCase::Integration, TESTLIB.to_string()))
}

pub(crate) trait TestlibModuleBuilder {
    fn testlib_module_defaults(self) -> Self;
}

impl<'func> TestlibModuleBuilder for testing::ModuleBuilder<'func> {
    fn testlib_module_defaults(self) -> Self {
        bak8::log::Log::init(None, None);
        //std::env::set_var("BAK8_TEST", "1");//todo
        self.import_fixture_dir(testlib_namepath())
            .base_temp_dir(env!("CARGO_TARGET_TMPDIR"))
    }
}

pub(crate) fn bak8_backup(cli: &bak8::cli::Cli, config: &bak8::config::BackupConfig) -> bak8::job::JobResults {
    bak8::run::backup::run_backup(&cli, &bak8::cli::BackupCommand::Scheduled, Some(config))
}

pub(crate) fn setup_backup_dir(test: &testing::Test, config: &bak8::config::BackupConfig) {
    bak8::Bak8Path::StorageDir(test.temp_dir().join(STRG_BAK8)).setup(config).unwrap();
}

pub(crate) fn make_scheduled_backup_cli(_test: &testing::Test) -> bak8::cli::Cli {
    bak8::cli::Cli {
        config_file: None,
        force: false,
        quiet: false,
        subcommand: bak8::cli::Command::Backup(bak8::cli::BackupCommand::Scheduled),
    }
}

pub(crate) fn make_config(test: &testing::Test, source_version: u8) -> bak8::config::BackupConfig {
    let username = bak8::sys::username();
    let usergroup = bak8::sys::usergroup();
    bak8::config::BackupConfig {
        backup_storage_dir: test.temp_dir().join(STRG_BAK8)
            .to_str().unwrap().to_string(),
        storage_admin_user: username.to_string(),
        backup_users_group: usergroup.to_string(),
        schedules: vec![
            bak8::config::BackupConfigSchedule {
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
            bak8::config::BackupConfigBackup {
                name: "home".to_string(),
                source_dir: test.imported_fixture_dir(&testlib_namepath())
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
    sleep_if_top_of_hour();

    let config = make_config(test, 1);
    setup_backup_dir(test, &config);

    let cli = make_scheduled_backup_cli(test);
    let mut results = bak8_backup(&cli, &config).unwrap();
    assert_eq!(2, results.len());
    results.pop().unwrap();
    let backup_output = match results.pop().unwrap() {
        bak8::JobOutput::Backup(job) => job, _ => panic!() };

    let earlier_run_name = bak8::backup::BackupRunName::new(
        chrono::Local::now().checked_sub_signed(chrono::Duration::minutes(1)).unwrap(),
        bak8::sys::hostname(),
        bak8::sys::username(),
        &config.backups[0].name,
    );

    let earlier_backup_run_dir = bak8::paths::Bak8Path::backup(
        test.temp_dir().join(STRG_BAK8),
        bak8::backup::BackupType::Full,
        &earlier_run_name);

    fs::rename(&backup_output.dest_dir, earlier_backup_run_dir).unwrap();
}

/// Remote testing need either "sshd.test" or localhost to be configured for SSH pubkey login (~/.ssh/config)
/// We will try to resolve "sshd.test" first and then fallback to localhost
pub(crate) fn resolve_test_remote() -> &'static bak8::Remote {
    static TEST_REMOTE: OnceLock<bak8::Remote> = OnceLock::new();
    TEST_REMOTE.get_or_init(|| {
        let host = match std::net::TcpStream::connect("sshd.test:22") {
            Ok(_) => "sshd.test".to_string(),
            Err(_) => {
                eprintln!("WARNING: Could not connect to 'sshd.test:22'. Falling back to 'localhost'");
                "localhost".to_string()
            }
        };

        bak8::Remote {
            name: "sshd-test".to_string(),
            host: host.clone(),
            user: None,
            platform: bak8::Platform::GNU,
        }
    })
}

/// Creates the necessary backup directories on the remote
pub(crate) fn setup_test_remote(config: &bak8::config::BackupConfig) -> bak8::config::BackupConfigRemote {
    let remote = resolve_test_remote();
    let temp_dir = bak8::cmd::ssh_run::ssh_temp_dir(&remote).unwrap();
    let storage_dir = bak8::Bak8Path::RemoteStorageDir { remote: remote.clone(), storage_dir: temp_dir.join(STRG_BAK8) };

    storage_dir.setup(config).unwrap();

    bak8::config::BackupConfigRemote {
        name: remote.name.clone(),
        host: remote.host.clone(),
        user: remote.user.clone(),
        platform: remote.platform.clone(),
        backup_storage_dir: storage_dir.as_path().to_str().unwrap().to_string(),
    }
}

pub(crate) fn teardown_test_remote(cfg_remote: &bak8::config::BackupConfigRemote) {
    let dir = PathBuf::from(&cfg_remote.backup_storage_dir)
        .parent().unwrap()
        .parent().unwrap().to_path_buf();

    // prevent disaster on test system
    let parent_dir = dir.parent().unwrap();
    assert_eq!(parent_dir, PathBuf::from("/tmp"), "DANGER: Attempting to delete non-temporary directory!");

    bak8::cmd::ssh_run::ssh_delete_dir(
        &cfg_remote.into(),
        &dir
    ).unwrap();
}

pub(crate) fn make_sync_config(
    test: &testing::Test,
    cfg_remote: &bak8::config::BackupConfigRemote,
    source_version: u8
) -> bak8::config::BackupConfig {
    let mut config = make_config(test, source_version);
    config.remotes = vec![ cfg_remote.clone() ];

    config.backups[0].syncs = vec![
        bak8::config::BackupConfigSync {
            remote: Some("sshd-test".to_string()),
            remote_group: None,
            sync_full: true,
            sync_incremental: true,
            sync_archive: true
        }
    ];

    config
}

pub(crate) fn expected_backup_ouput_dir(test: &testing::Test, backup_type: bak8::BackupType, output: &bak8::BackupJobOutput
) -> PathBuf {
    test.temp_dir()
        .join(STRG_BAK8)
        .join(backup_type.subdir_name())
        .join(whoami::hostname().unwrap())
        .join(whoami::username().unwrap())
        .join(output.run_name.datetime.format("%Y").to_string())
        .join(output.run_name.datetime.format("%m").to_string())
        .join(output.run_name.datetime.format("%d").to_string())
        .join(output.run_name.to_string())
}

pub(crate) fn assert_remote_backup_synced(
    test: &testing::Test,
    backup_type: bak8::BackupType,
    source_num: u8,
    remote_cfg: &bak8::config::BackupConfigRemote,
    output: &bak8::SyncBackupJobOutput
) {
    assert_eq!(remote_cfg.name, output.remote.name);

    let expected_dir = PathBuf::from(&remote_cfg.backup_storage_dir)
        .join(backup_type.subdir_name())
        .join(whoami::hostname().unwrap())
        .join(whoami::username().unwrap())
        .join(output.backup_run_name.datetime.format("%Y").to_string())
        .join(output.backup_run_name.datetime.format("%m").to_string())
        .join(output.backup_run_name.datetime.format("%d").to_string())
        .join(output.backup_run_name.to_string());
    assert_eq!(expected_dir, output.remote_dest_dir.as_path());

    // remotely test to see if the expected dir exists
    assert!(bak8::cmd::ssh_run::ssh_dirs_exist(&output.remote, &vec![&expected_dir]).unwrap());

    // compare sha256sum manifest between testing source and remote destination
    let remote_manifest = bak8::cmd::ssh_run::ssh_dir_manifest(&output.remote, &expected_dir).unwrap();
    let expected_manifest = fs::read_to_string(test.imported_fixture_dir(testlib_namepath())
        .join(MOCK_FS_DIRNAME)
        .join(format!("{}{source_num}", SOURCE_PREFIX))
        .with_extension("manifest")).unwrap().trim().to_string();
    assert_eq!(expected_manifest, remote_manifest);
}

pub(crate) fn assert_remote_archive_synced(
    _test: &testing::Test,
    remote_cfg: &bak8::config::BackupConfigRemote,
    archive_output: &bak8::ArchiveJobOutput,
    output: &bak8::SyncArchiveJobOutput
) {
    assert_eq!(remote_cfg.name, output.remote.name);

    let expected_filepath = PathBuf::from(&remote_cfg.backup_storage_dir)
        .join(bak8::paths::consts::BACKUP_ARCHIVE_DIRNAME)
        .join(whoami::hostname().unwrap())
        .join(whoami::username().unwrap())
        .join(output.backup_run_name.datetime.format("%Y").to_string())
        .join(output.backup_run_name.datetime.format("%m").to_string())
        .join(output.backup_run_name.datetime.format("%d").to_string())
        .join(output.backup_run_name.to_string())
        .with_extension(bak8::paths::consts::TAR_XZ_EXTENSION);
    assert_eq!(expected_filepath, output.remote_dest_filepath.as_path());

    let expected_checksum_filepath = PathBuf::from(&remote_cfg.backup_storage_dir)
        .join(bak8::paths::consts::BACKUP_ARCHIVE_DIRNAME)
        .join(whoami::hostname().unwrap())
        .join(whoami::username().unwrap())
        .join(output.backup_run_name.datetime.format("%Y").to_string())
        .join(output.backup_run_name.datetime.format("%m").to_string())
        .join(output.backup_run_name.datetime.format("%d").to_string())
        .join(output.backup_run_name.to_string())
        .with_extension(format!("{}.{}", bak8::paths::consts::TAR_XZ_EXTENSION, bak8::paths::consts::SHA256_EXTENSION));
    assert_eq!(expected_checksum_filepath, output.remote_dest_checksum_filepath.as_path());

    // remotely test to see if the expected dir exists
    assert!(bak8::cmd::ssh_run::ssh_files_exist(&output.remote, &vec![
        &expected_filepath,
        &expected_checksum_filepath]).unwrap());

    // compare sha256sum manifest between testing source and remote destination
    let remote_checksum = bak8::cmd::ssh_run::ssh_file_contents(&output.remote, &expected_checksum_filepath)
        .unwrap().unwrap()
        .split_ascii_whitespace()
        .next().unwrap().to_string();
    assert_eq!(archive_output.checksum, remote_checksum);
}
