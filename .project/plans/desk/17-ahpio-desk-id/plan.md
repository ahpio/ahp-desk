---
title: The desk is com.ahpio.desk, and a desk updated from 0.2.3 keeps its hosts and its ahpd
domain: desk
status: active
priority: high
created: 2026-11-01
revalidated: 2026-11-01
requires:
  - plans/desk/03-local-ahpd/plan.md
changes:
  - specs/host-list.md
decisions:
  - decisions/the-desks-identifier-is-com-ahpio-desk.md
  - decisions/a-desk-updated-from-0-2-3-copies-its-config-and-moves-its-local-data.md
refs:
  - "[code://src-tauri/tauri.conf.json#L5](../../../../src-tauri/tauri.conf.json#L5) - `identifier`"
  - "[code://src-tauri/src/lib.rs#L60-L72](../../../../src-tauri/src/lib.rs#L60-L72) - the setup that loads `hosts.json` and the settings from the app config folder"
  - "[code://src-tauri/src/local_ahpd.rs#L133-L139](../../../../src-tauri/src/local_ahpd.rs#L133-L139) - `LocalAhpd::new`, the ahpd prefix in the app local data folder"
  - "[code://src-tauri/src/tokens.rs#L20](../../../../src-tauri/src/tokens.rs#L20) - the keyring service `ahp-desk`, not named after the identifier"
  - "[code://src-tauri/windows/hooks.nsh](../../../../src-tauri/windows/hooks.nsh) - the installer hooks, where an uninstall with the data removes the old folders"
  - "[code://README.md#L131](../../../../README.md#L131) - where the desk keeps things"
  - https://github.com/tauri-apps/tauri/blob/@tauri-apps/cli-v2.12.1/crates/tauri-bundler/src/bundle/windows/nsis/installer.nsi - the uninstall key is named after the product, not the identifier, so an update from 0.2.3 still finds the old install
---

## Goal

The desk's identifier becomes `com.ahpio.desk`, after the repository moved to the ahpio organization.
A desk that updates from 0.2.3 or earlier starts with the hosts, settings and trusted SSH keys it had, and with the ahpd it installed.

## Reconnaissance

The files read and the patterns to reuse are the `refs` above, each with its note.

### Searches performed

- `rg "app_config_dir|app_local_data_dir|app_data_dir" src-tauri/src` - the config folder in `lib.rs` setup, the local data folder in `LocalAhpd::new`; nothing else names a folder after the identifier.
- `rg "keyring::Entry" src-tauri/src` - the service is the const `ahp-desk`.
- The NSIS template at `@tauri-apps/cli` 2.12.1 - `UNINSTKEY` uses `PRODUCTNAME`; `MANUFACTURER` comes from the identifier when no publisher is set; an uninstall with "delete the app data" removes `$APPDATA\${BUNDLEID}` and `$LOCALAPPDATA\${BUNDLEID}` only.

### Runtime path

```
desk start -> setup -> (new) old_id::take_old_folders(config, local) -> HostStore::load(config/hosts.json) -> LocalAhpd::new(local/ahpd)
```

### Gaps

- Nothing reads the folders of the old identifier.
- An uninstall with the data leaves the old folders and `HKCU\Software\softov\ahp-desk`.

## Decisions locked in

| # | Decision | Rationale / source |
| --- | --- | --- |
| 1 | [The desk's identifier is com.ahpio.desk](../../../decisions/the-desks-identifier-is-com-ahpio-desk.md) | Softov, 2026-11-01: "lets use com.ahpio.desk" |
| 2 | [A desk updated from 0.2.3 copies its old config folder and moves its old local data folder](../../../decisions/a-desk-updated-from-0-2-3-copies-its-config-and-moves-its-local-data.md) | Softov, 2026-11-01: "Copy the old folder (Recommended)", "Move it (Recommended)", "Copy, keep the old (Recommended)" |

| What | Source | Task |
| --- | --- | --- |
| The change ships as 0.3.0, with the links to ahpio/ahp-desk and the Linux npm fix. | Softov, 2026-11-01: "0.3.0: ID + links + Linux fix (Recommended)" | - |
| A failure to copy or move is logged and the desk starts without the old folders. | (defaulted: a desk that does not start is worse than one that asks for its hosts again) | 01 |
| Nothing is copied or moved when `AHP_DESK_CONFIG_DIR` is set. | (defaulted: a test run never touches the user's folders) | 01 |
| The copy is made in `dev.softov.ahpdesk.copying` beside the new folder and renamed into place. | (defaulted: a stop halfway never leaves half a host list) | 01 |
| An uninstall with the data on Windows removes the old folders and `HKCU\Software\softov\ahp-desk`. | (defaulted: the copy kept for 0.2.3 would otherwise stay forever) | 01 |
| No publisher is set in `tauri.conf.json`; the installer's publisher reads `ahpio`. | (defaulted: the publisher follows the code signing identity, not settled yet) | - |

## Proposed architecture

- **Data flow** - `take_old_folders` copies `<config base>/dev.softov.ahpdesk` to `<config base>/com.ahpio.desk` and renames `<local base>/dev.softov.ahpdesk` to `<local base>/com.ahpio.desk`, each when the new one is missing or empty.
- **Event flow** - none.
- **State flow** - none new; the files keep their shape.
- **Layer responsibilities** - `src-tauri`: the identifier, the copy and the move, the uninstall hook · docs: README's folders.
- **Source-of-truth files** - [`code://src-tauri/src/old_id.rs`](../../../../src-tauri/src/old_id.rs).

## Tasks

| Task | Status | Depends on |
| --- | --- | --- |
| [01 - the identifier and the old folders](task-01-the-identifier-and-the-old-folders.md) | done | - |
| [02 - an update from 0.2.3 by hand on Windows and Debian](task-02-an-update-from-0-2-3-by-hand.md) | todo | 01 |

## Risks and tradeoffs

- A host whose `ahpd_path` was set by hand inside the old local data folder points at a path that moved - Change picks it again; the desk's own ahpd is found without a set path.
- On Linux, a desk-installed ahpd keeps running from the old path after the rename, since an open file survives it - its next start runs from the new path.
- The single-instance lock is named after the identifier, so a 0.2.3 desk still open beside a 0.3.0 one is not seen - the update closes the old desk on Windows; on Linux the updater restarts it.

## Resume state

- **Done so far:** task 01, 2026-11-01.
- **Next action:** [task-02-an-update-from-0-2-3-by-hand.md](task-02-an-update-from-0-2-3-by-hand.md), with an installer built from this branch, before the 0.3.0 tag.
- **Open questions:** none.
- **Watch out for:** test with a 0.2.3 that has hosts and a desk-installed ahpd, not a fresh one; a fresh desk has no old folders.

## Final verification checklist

- [x] `pnpm typecheck`, `pnpm test` and `cargo clippy --all-targets -- -D warnings` pass.
- [ ] By hand on Windows: 0.2.3 with a host and its ahpd, then this build installed over it, keeps the host and the ahpd.
- [ ] By hand on Debian: the same with the deb.
- [x] `specs/host-list.md` corrected.
- [x] `plans/index.md` updated.
