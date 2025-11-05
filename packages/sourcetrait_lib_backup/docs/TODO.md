TODO: Bak8 CLI
===============================================================================

## fix: Setup doesn't create subdirs for local system or bak8 user

Breaks runs. Attributes should be appropriate for each subdir.

## feat: Config for running custom prep scripts

1. Prep for full backups
2. Prep for incremental backups

Prep scripts should NOT break the backup if they exit in error.

Only STDERR should be saved to logs.

The user report should be warned if any STDERR was received.

## feat: Automatic recovery from certain problems

Re-attempt to perform these tasks, if they have failed, on the next run of a backup:
  - Unable to upload backup
  - Unable to upload archive

## feat: Remote setup / install

Setup a remote (SSH) system for receiving backup uploads.

