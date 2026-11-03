# CHANGELOG — Change Log

English · [Español](../es/CHANGELOG.md) · [简体中文](../CHANGELOG.md)

## V1.3 — 2026-11-04

### Fixed
- Moved the broker working directory back to `/data/local/tmp`, fixing the V1.2 real-device regression where responses were empty (SELinux labels).

## V1.2 — 2026-11-03

### Changed
- Engine security hardening: unknown Android versions no longer panic, `keymint.dat` is written atomically, and a failed keybox parse no longer overwrites the file on disk.

## V1.1 — 2026-11-03

### Changed
- Adaptive polling in the broker (almost no wait during consecutive operations, backs off when idle).
- Core shell hardening: response and pairing token writes use O_EXCL, Settings key names are validated, and `heraldctl log` reads through the broker.

## V1.0 — 2026-11-01

### Added
- First release: broker, `heraldctl`, plugin interface, and the authentication engine (keymint, inject) forked from Oh My Keymint.
