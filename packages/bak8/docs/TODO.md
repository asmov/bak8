TODO: Bak8 CLI
===============================================================================

## fix: Setup doesn't create subdirs for local system or bak8 user

Breaks runs. Attributes should be appropriate for each subdir.

## feat: Config for ignored paths (rsync)

User sets up various paths to ignore in their config. Rsync exclused each one.

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

## feat: Design reporting

Reports: How do we do?

Cloud reporting will probably be a main source of monetization to support this project, so do it well.

Desktop: A quick little pop-up when the user logs in for the first time that morning.
Email: On warnings and errors.
Email: Summary report email every so often (weekly default).

Summary should show good, bad, space used, space remaining, change in space used (trend).
