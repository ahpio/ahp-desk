---
title: The desk's identifier is com.ahpio.desk
status: accepted
date: 2026-11-01
refs:
  - "[code://src-tauri/tauri.conf.json#L5](../../src-tauri/tauri.conf.json#L5) - `identifier`, which names the desk's config and local data folders"
  - https://github.com/ahpio/ahp-desk - the repository, moved from softov to the ahpio organization on 2026-11-01
---

## Context

Up to 0.2.3 the identifier was `dev.softov.ahpdesk`, from a personal domain.
The repository moved to the ahpio organization, and the desk is published under ahpio.com.
The identifier names the folders the desk keeps its data in, so a change must bring the old folders over (decision `a-desk-updated-from-0-2-3-copies-its-config-and-moves-its-local-data`).

## Decision

The identifier is `com.ahpio.desk`, from the 0.3.0 release on.
Source: Softov, 2026-11-01, offered `com.ahpio.desk` or `com.ahpio.ahp-desk`: "lets use com.ahpio.desk".

## Consequences

The config folder is `%APPDATA%\com.ahpio.desk` on Windows and `~/.config/com.ahpio.desk` on Linux; the local data folder moves the same way.
The NSIS installer's publisher, taken from the identifier, reads `ahpio`, and its install key is `HKCU\Software\ahpio\ahp-desk`.
The keyring service stays `ahp-desk`, so tokens and passwords are untouched.
Other ahpio apps can take `com.ahpio.<name>`.

## Options

- **`com.ahpio.ahp-desk`.** Lost: some systems refuse a hyphen in an identifier, and the name repeats the domain.
- **Keep `dev.softov.ahpdesk`.** Lost: the published app keeps a personal domain after the move to ahpio.
