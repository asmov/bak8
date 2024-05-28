DESIGN: bak8
===============================================================================

Bak8 is a backup automation suite with a strong focus on backing up $HOME
directories on desktop systems.

We have a primary focus on Linux desktop systems, with secondary support for
MacOS and Windows (via WSL).

We ship a main CLI executable, 'bak8' that can be used to run both manual and
scheduled (automatic) backups. Scheduling currently supports systemd.

There is an optional GUI desktop executable 'bak8-desktop', that uses libcosmic
on Linux and Tauri + Yew for everything else. It is used mainly for
visualization of reports and configuration.

We also ship a mobile app, 'Bak8 Mobile', that is used primarily to receive
reports remotely.

Finally, we provide a standalone CLI tool, 'bak', that allows for simple
adhoc backups of single files, typically for use in system administration. It is
not directly related to the rest of bak8 and provided merely as a useful tool.


Support
-------------------------------------------------------------------------------

Bak8 has tiered multi-platform support:
1. Linux Primary
  - Desktop: COSMIC
  - Server: Ubuntu Server
2. Linux Server Secondary
  - Server: Red Hat
3. Desktop Secondary (via Tauri + Yew)
  - Linux: GNOME and KDE (via libcosmic)
  - Windows 11+ (using bak8 via WSL)
  - MacOS 15+ 

Bak8 Mobile has tiered multi-platform support:
1. Android (Primary)
2. iOS (via Tauri + Yew)
3. Mobile Web (via Yew)
