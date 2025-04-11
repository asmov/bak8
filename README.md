Bak8
================================================================================
*Rotational backup system for workstation users*

**Bak8** is a rotational backup system focused on providing workstation users
with easily accessible daily backups of their home directories. Backups
can be configured to be archived, encrypted, and securely synchronized
across multiple devices and cloud storage providers.

The Bak8 software suite includes the following filesystem backup tools:
- [bak8](#bak8) Manages a rotational backup system
- [bak](#bak) Creates an adhoc `.bak` copy of a file


[bak8](./packages/bak8/)
--------------------------------------------------------------------------------
[![Latest Version: bak8]][crates.io:bak8]

[Latest Version: bak8]: https://img.shields.io/crates/v/bak8.svg
[crates.io:bak8]: https://crates.io/crates/bak8

### Usage

`bak8 [OPTIONS] <COMMAND>`

Manages a rotational backup system.

Refer to the [bak8 project](./packages/bak8/) for more information.

[bak](./packages/bak8-bak/)
--------------------------------------------------------------------------------
[![Latest Version: bak8-bak]][crates.io:bak8-bak]

[Latest Version: bak8-bak]: https://img.shields.io/crates/v/bak8-bak.svg
[crates.io:bak8-bak]: https://crates.io/crates/bak8-bak

### Usage

`bak [OPTIONS] FILE [DIR] [COMMAND]`

Creates an adhoc backup `.bak` copy of **FILE**.

Refer to the [bak project](./packages/bak8-bak/) for more information.

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
*bak8* is a pending trademark of Asmov LLC.
