---
title: The identifier is com.ahpio.desk, and the old folders come over on the first start
status: done
depends: []
layer: "src-tauri, docs"
refs:
  - "[code://src-tauri/src/lib.rs#L60-L72](../../../../src-tauri/src/lib.rs#L60-L72) - the setup, before `HostStore::load`"
  - "[code://src-tauri/windows/hooks.nsh](../../../../src-tauri/windows/hooks.nsh) - `NSIS_HOOK_POSTUNINSTALL`"
---

## Objective

The desk runs as `com.ahpio.desk`, and on its first start copies the old config folder and moves the old local data folder (decisions 1 and 2).

## Files

- `CREATE: src-tauri/src/old_id.rs` - `take_old_folders`, `copy_config`, `move_local_data`, and their tests.
- `UPDATE: src-tauri/src/lib.rs` - `mod old_id`; the setup calls `take_old_folders` when `AHP_DESK_CONFIG_DIR` is not set.
- `UPDATE: src-tauri/tauri.conf.json:5` - `identifier` is `com.ahpio.desk`.
- `UPDATE: src-tauri/windows/hooks.nsh` - an uninstall with the data removes `$APPDATA\dev.softov.ahpdesk`, `$LOCALAPPDATA\dev.softov.ahpdesk` and `HKCU\Software\softov\ahp-desk`.
- `UPDATE: README.md:131` - the new folder, and the copy from the old one.
- `UPDATE: .project/specs/host-list.md` - the scenario "A desk updated from 0.2.3 keeps its hosts and its ahpd".

## Steps

1. `old_beside(new)` answers `<parent>/dev.softov.ahpdesk` when it is a folder and `new` is missing or empty.
2. `copy_config` copies it into `dev.softov.ahpdesk.copying` (removed first if a stop left one), removes an empty `new`, and renames the copy to `new`.
3. `move_local_data` removes an empty `new` and renames the old folder to it.
4. `take_old_folders` runs both and logs each failure with `eprintln!`.

## Validation

- `old_id.rs` tests: the config is copied and the old stays; the local data is moved; an empty new folder is taken over; a new folder with a file is left alone; with no old folder nothing is made; a copy left halfway is made again.
- `pnpm typecheck`, `pnpm test` (with `CARGO_TARGET_DIR=F:/github/ahp-desk/src-tauri/target`) and `cargo clippy --all-targets -- -D warnings` pass.

## Resume

2026-11-01: done; the checks pass.
