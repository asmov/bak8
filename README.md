SourceTrait Backup
================================================================================
*Rotational backup system for workstation users*

**SourceTrait Backup** is a rotational backup system focused on providing workstation users
with easily accessible daily backups of their home directories. Backups
can be configured to be archived, encrypted, and securely synchronized
across multiple devices and cloud storage providers.

The SourceTrait Backup software suite includes the following filesystem backup tools:
- [backup](#backup) Manages a rotational backup system
- [bak](#bak) Creates an adhoc `.bak` copy of a file


[backup](./packages/backup/)
--------------------------------------------------------------------------------
[![Latest Version: backup]][crates.io:backup]

[Latest Version: sourcetrait_backup]: https://img.shields.io/crates/v/sourcetrait_lib_backup.svg
[crates.io:sourcetrait_backup]: https://crates.io/crates/sourcetrait_lib_backup

### Usage

`sourcetrait backup [OPTIONS] <COMMAND>`

Manages a rotational backup system.

Refer to the [sourcetrait backup project](./packages/sourcetrait_lib_backup/) for more information.

[bak](./packages/sourcetrait_lib_bak/)
--------------------------------------------------------------------------------
[![Latest Version: sourcetrait_lib_bak]][crates.io:sourcetrait_lib_bak]

[Latest Version: sourcetrait_lib_bak]: https://img.shields.io/crates/v/sourcetrait_lib_bak.svg
[crates.io:sourcetrait_lib_bak]: https://crates.io/crates/sourcetrait_lib_bak

### Usage

`bak [OPTIONS] FILE [DIR] [COMMAND]`

Creates an adhoc backup `.bak` copy of **FILE**.

Refer to the [bak project](./packages/sourcetrait_lib_bak/) for more information.

License (AGPL3)
--------------------------------------------------------------------------------
SourceTrait Backup: Rotational backup system for workstation users  
Copyright (C) 2024-2025 Asmov LLC  

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a [copy](./LICENSE-AGPL-3.txt) of the GNU Affero General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.
