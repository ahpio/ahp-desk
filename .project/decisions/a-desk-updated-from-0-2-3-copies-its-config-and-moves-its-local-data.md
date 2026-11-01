---
title: A desk updated from 0.2.3 copies its old config folder and moves its old local data folder
status: accepted
date: 2026-11-01
refs:
  - "[code://src-tauri/src/old_id.rs](../../src-tauri/src/old_id.rs) - `take_old_folders`"
  - "[code://src-tauri/src/lib.rs#L60-L72](../../src-tauri/src/lib.rs#L60-L72) - the setup that calls it before anything reads the folders"
  - "[code://src-tauri/src/local_ahpd.rs#L133-L139](../../src-tauri/src/local_ahpd.rs#L133-L139) - the ahpd the desk installs, in `<app local data>/ahpd`"
---

## Context

The identifier changes from `dev.softov.ahpdesk` to `com.ahpio.desk` (decision `the-desks-identifier-is-com-ahpio-desk`).
The config folder holds `hosts.json`, `settings.json`, `updates.json` and `known_keys.json`, a few small files.
The local data folder holds the ahpd the desk installed, an npm prefix often over 100 MB, and the app windows' web storage.
Without a move, a desk that updates starts with no hosts and offers to install ahpd again.

## Decision

On its first start as `com.ahpio.desk`, the desk copies the old config folder into the new one and keeps the old one.
It moves the old local data folder to the new one with a rename.
Each happens only when the new folder is missing or empty and the old one is there.
Source: Softov, 2026-11-01, asked "What should happen to an existing user's hosts after they update?": "Copy the old folder (Recommended)"; asked about the local data folder: "Move it (Recommended)"; asked whether the old config folder stays: "Copy, keep the old (Recommended)".

## Consequences

A desk that goes back to 0.2.3 still finds its hosts, but not the ahpd it installed, which moved.
The rename is instant on the same drive and leaves no second copy of the ahpd.
Removing the desk on Windows with its data also removes the old folders.

## Options

- **Start fresh.** Lost: every user adds their hosts again and installs ahpd again.
- **Copy the local data folder too.** Lost: a slow first start and a second copy of the ahpd until someone removes it by hand.
- **Move the config folder too.** Lost: going back to 0.2.3 would find no hosts.
