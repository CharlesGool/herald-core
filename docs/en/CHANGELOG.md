# CHANGELOG — Change Log

English · [Español](../es/CHANGELOG.md) · [简体中文](../CHANGELOG.md)

## V1.5 — 2026-11-04

### Removed
- Removed all SOTER (Qualcomm/Tencent) implementations: injector `hook/soter.rs` and `soter_watch.rs`, keymint `soter_beta.rs` and `soter_hal.rs`, the `soter-ta` crate, and pif-common `soter.rs` and `soter_blob.rs` (not enabled on TB321FU).
- Dropped the AGPL-3.0 that came with it: AGPL was only introduced by ommega's soter-ta. Deleted `engine/LICENSE-AGPL-3.0.txt` and `licenses/ommega-soter-ta.txt`, and updated NOTICE, THIRD_PARTY_NOTICES and README accordingly.

### Fixed
- resetprop's dangling call to the removed `--soter-watch`: it would re-inject keystore2 every 60 seconds (a HIGH defect exposed by removing SOTER).

### Changed
- The injector gains `inject --target keystore2|system_server` target selection, and `entry` picks its extension by process name. The keystore authentication path is byte-for-byte unchanged, and the HERALD RPC session is only established in the keystore branch.
- Added a pass-through `PkgManagerInterceptor` (P2.0, not triggered by default) as low-level validation for app-list hiding.

### Review
- P2 (injecting into system_server to intercept `IPackageManager` for app-list hiding) was blocked by SELinux after real-device de-risking: with system_server enforcing, it cannot load the `herald_exec` injection body, and letting it through would hit neverallow and weaken security. It is **shelved**; app-list hiding moves to `services.jar`. See `docs/injector-capability-design.md`.

## V1.4 — 2026-11-04

### Added
- broker plugin interface gains `HERALDP_ASYNC`: plugins can declare slow commands to run in the background without blocking the main loop (optional; the default empty set keeps behavior unchanged).

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
