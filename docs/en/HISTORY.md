# HISTORY — History

English · [Español](../es/HISTORY.md) · [简体中文](../HISTORY.md)

- 2026-11-01: Project created. The core (broker + heraldctl + plugin interface) was developed in-house. The authentication engine was forked from Oh My Keymint (baseline bafe5ba) and renamed to herald. Original name `omk-core`, later changed to `herald-core`.
- 2026-11-04: After the working-directory real-device regression fix, V1.3 was archived alongside the TB321FU `user-v1.3` official release.
- 2026-11-04: V1.5 removed SOTER (along with the AGPL) and fixed the dangling resetprop `--soter-watch` call (HIGH). The injector gained capability-based P2.0 target selection and pass-through. P2 (injecting into system_server to hide apps from the app list) was blocked by SELinux according to real-device verification, so it was shelved and app-list hiding moves to services.jar. Archived as the TB321FU `user-v1.4` anchor official release.
