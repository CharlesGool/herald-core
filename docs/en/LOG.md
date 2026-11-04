# LOG — Maintenance Log

English · [Español](../es/LOG.md) · [简体中文](../LOG.md)

## Records

For version changes, see [CHANGELOG.md](CHANGELOG.md); for historical requirements and old handoffs, see [HISTORY.md](HISTORY.md).

## Defects

None at present.

## Limitations

- The `keymint`/`inject` binaries in `engine/` are subject to GLA v4 and are restricted to non-commercial use.

## Decisions

| Date | Decision | Status | Reason |
|---|---|---|---|
| 2026-11-01 | Renamed omk→herald across the whole engine | Accepted | Clause 3 of the Oh My Keymint License does not allow the original name to be used beyond its permitted scope |
| 2026-11-04 | Broker working directory stays in `/data/local/tmp` | Accepted | Moving it to `/data/adb` makes responses unreadable due to SELinux labels (V1.2 real-device regression) |
| 2026-11-04 | Removed SOTER and dropped the AGPL that came with it | Accepted | TB321FU does not enable SOTER; AGPL was only introduced by ommega's soter-ta, so it was removed together |
| 2026-11-04 | App-list hiding does not go through injection into system_server | Accepted | system_server (enforcing) cannot load the herald_exec injection body; letting it through would hit neverallow and weaken security. Moved to services.jar (confirmed by real-device de-risking) |

## Handoff

No work in progress found. Current branch `main`, in sync with `origin/main`.
