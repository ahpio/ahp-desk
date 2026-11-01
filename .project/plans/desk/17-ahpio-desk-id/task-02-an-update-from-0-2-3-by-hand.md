---
title: An update from 0.2.3 keeps the hosts and the ahpd, by hand on Windows and Debian
status: todo
depends: [task-01-the-identifier-and-the-old-folders.md]
layer: "by hand"
refs:
  - "[code://DEVELOPER.md](../../../../DEVELOPER.md) - the toolchain and `pnpm tauri build`"
---

## Objective

On the Windows test VM and the Debian test box, a desk 0.2.3 with a host and a desk-installed ahpd, updated to a build of this branch, starts with the same host list and the same ahpd.

## Files

- none.

## Steps

1. Windows: with desk 0.2.3 installed, add This computer and let the desk install ahpd; add a URL host too.
2. Windows: quit the desk, run the NSIS installer built from this branch, start the desk.
   The two hosts are there, This computer's ahpd starts with no install offered, `%APPDATA%\dev.softov.ahpdesk` still holds `hosts.json`, and `%LOCALAPPDATA%\dev.softov.ahpdesk` is gone.
3. Windows: uninstall with "delete the app data": both `com.ahpio.desk` and both `dev.softov.ahpdesk` folders are gone, and so is `HKCU\Software\softov\ahp-desk`.
4. Debian: the same as steps 1 and 2 with the 0.2.3 release deb and a deb built from this branch, with `~/.config` and `~/.local/share`.
5. Put both machines back as they were.

## Validation

- Each step's result written in this task's Resume.

## Resume

