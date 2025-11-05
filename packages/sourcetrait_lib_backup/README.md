SourceTrait Backup
================================================================================
[![Latest Version: sourcetrait_backup]][crates.io:sourcetrait_backup]

[Latest Version: sourcetrait_backup]: https://img.shields.io/crates/v/sourcetrait_lib_backkup.svg
[crates.io:sourcetrait_backup]: https://crates.io/crates/sourcetrait_lib_backup

*Rotational backup system for workstation users*  

*$HOME is where the heart is.*

## Usage

`sourcetrait backup [OPTIONS] <COMMAND>`

Manages a rotational backup system.

### Commands
- `backup`   Performs backups as configured
- `config`   Manages configuration
- `log`      Reviews logs
- `summary`  Reviews a summary of recent backups

### `sourcetrait backup` `<SUBCOMMAND> <NAME>`

`NAME`: The name of the backup configuration to operate on.

#### Subcommands:
- `scheduled` Performs backups as schedled
- `full` Manually performs a full backup
- `incremental` Manually performs an incremental backup

### `sourcetrait backup config` `<SUBCOMMAND>`

#### Subcommands
- `setup` Initializes the user's bak8 configuration
- `edit` Opens the bak8 configuration file in their editor
- `verify` Verifies configuration
- `show` Displays the configuration file contents


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

