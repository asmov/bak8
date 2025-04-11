DESIGN: Bak8
===============================================================================

**Bak8** is a rotational backup automation toolset with a focus on protecting
user data on desktop operating systems.

The primary OS target is Linux, with MacOS and Windows (WSL) as secondary
targets.

The Bak8 suite consists of two CLI tools:
- bak8
- bak

The flagship tool, `bak8` can be used to run both manual and scheduled
backups based on configuration that the user provides. 

We also ship `bak`, which simply creates adhoc `.bak` copies of files,
typically for use in basic system administration and development.

Bak8
--------------------------------------------------------------------------------

Under the hood, `bak8` is essentially an automation tool for `rsync`.

The user maintains a configuration file that defines:
- which directories on the filesystem to archive,
- when backups should run,
- how many incremental backups should be made before another full backup,
- how to zip backups,
- how to encrypt zipped backups,
- which remote servers should backups to be sync'd to (via SSH),
- and how many incremental and full backups to store before pruning.

With this configuration, normal usage of the bak8 command will
perform any operations necessary when ran, based on schedule. After the user
sets up their configuration, it's basically hands-off.

Most operations fork child processes to make use of system commands:
- `rsync`
- `tar` (with `xz`)
- `gpg`
- `ssh`



