bak8
================================================================================
[![Latest Version: bak8]][crates.io:bak8]

[Latest Version: bak8]: https://img.shields.io/crates/v/bak8.svg
[crates.io:bak8]: https://crates.io/crates/bak8

*Rotational backup system for workstation users*  

*$HOME is where the heart is.*

## Usage

`bak8 [OPTIONS] <COMMAND>`

Manages a rotational backup system.

### Commands
- `backup`   Performs backups as configured
- `config`   Manages configuration
- `log`      Reviews logs
- `summary`  Reviews a summary of recent backups

### `bak8 backup` `<SUBCOMMAND> <NAME>`

`NAME`: The name of the backup configuration to operate on.

#### Subcommands:
- `scheduled` Performs backups as schedled
- `full` Manually performs a full backup
- `incremental` Manually performs an incremental backup

### `bak8 config` `<SUBCOMMAND>`

#### Subcommands
- `setup` Initializes the user's bak8 configuration
- `edit` Opens the bak8 configuration file in their editor
- `verify` Verifies configuration
- `show` Displays the configuration file contents


Repository
--------------------------------------------------------------------------------
Contributors, please review [ASMOV.md](./ASMOV.md).  

Found a bug? Search for an existing issue on GitHub.  
If an issue exists, chime in to add weight to it.  
If an issue does not exist, create one and tell us how to reproduce the bug. 


License (AGPL3)
--------------------------------------------------------------------------------
bak8: Rotational backup system for workstation users  
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


Trademark
--------------------------------------------------------------------------------
*bak8* is a trademark of Asmov LLC.
