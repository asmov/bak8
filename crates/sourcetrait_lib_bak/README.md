SourceTrait CLI: Bak
================================================================================
[![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/sourcetrait_cli_bak.svg
[crates.io]: https://crates.io/crates/sourcetrait_cli_bak

Creates an adhoc backup `.bak` copy of a file.


Usage
--------------------------------------------------------------------------------

`bak [OPTIONS] FILE [DIR] [COMMAND]`

Creates an adhoc backup `.bak` copy of **FILE**.

If **DIR** is not specified, the copy is created in the same directory as FILE.

If DIR is specified as `-`, or if the user lacks permissions to copy to DIR, the
user's app data directory will be used instead.

If multiple backups of FILE exist, the rotating filename extension used will be: `.bak.N`

The most recent rotating backup will always be `.bak.0` 

Pruning of rotating backups occurs after `-n NUM` copies. 

If the current backup is no different than its predecessor, copying will be skipped. 

Additional **COMMAND**s may be appended to list, compare, or delete backups.

### Options

- `-n NUM`  
Creates at most **NUM** backup files. [default: 10] 

- `-q`  
Quiet. Suppresses output.

- `-f`
Force the operation without confirmation.

### Commands

- `ls`  
Lists all backups of FILE in DIR.

- `diff N`  
Shows the differences between FILE and the specified `bak.N` backup in DIR. [default: 0]

- `rm`  
Deletes all backups of FILE in DIR.

License (AGPL3)
--------------------------------------------------------------------------------
SourceTrait CLI: Bak: Creates an adhoc backup `.bak` copy of a file.  
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
