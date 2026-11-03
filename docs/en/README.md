# herald-core

English · [Español](../es/README.md) · [简体中文](../../README.md)

Rootless core: a broker running in its own SELinux domain, the command-line tool `heraldctl`, and a plugin interface that other patches connect to.
**The artifact is the GSI patch `herald-core`** (`make dist` → `dist/herald-core/`), used in TB321FU-GSI-Mods.

| Directory | Contents |
|---|---|
| `patch/` | Patch source; see `patch/README.md` for details |
| `docs/plugin-api.md` | Plugin interface description |
| `engine/` | Authentication engine (HeraldKeymint) source, forked from Oh My Keymint with identifiers changed to herald. Only the parts needed to build `keymint` and `inject` are kept. `make engine` builds it; see `engine/README.herald.md` for details |
| `engine/src/` | keymint daemon (KeyMint implementation replacing the keystore2 backend, keybox, local RKP) |
| `engine/injector/` | inject injector (ptrace-injects into keystore2, routes requests per App) |
| `engine/common/`, `boringssl/`, `ta/`, `wire/`, `derive/`, `watchdog/`, `tests/` | Base libraries such as the kmr-* crates from AOSP KeyMint |
| `engine/soter-ta/`, `pif-common/` | Qualcomm Soter software TA (adapted from ommega) and the shared Soter definitions |
| `engine/aidl/`, `proto/`, `runtime/`, `assets/` | AIDL interfaces, protobuf, the Java part of the RKP V1 bridge, and resource files compiled into the binary |
| `engine/scripts/` | `build-android.sh` (called by `make engine`), `build-boringssl.sh` (builds BoringSSL at a pinned version) |
| `engine/docs/` | Upstream docs (configuration notes, FAQ, third-party sources); sections specific to module mode are marked |

The parts of upstream that only serve Magisk/KernelSU module mode (`webui/`, `template/`, `pif-spoof/`, `hal/`, packaging scripts, etc.) have been removed. The reasons are in `engine/README.herald.md`.

**Relationship with TB321FU-GSI-Mods**: This repo is the single source of truth. The synced copy in Mods at `patches/required/general/herald-core` is a mirror. After changing things here, run `make sync-core` in Mods (`make unit` checks that both copies match).

## License

Several licenses coexist in this repo. Each governs its own part, and they do not override one another:

| Part | License | Full text |
|---|---|---|
| `patch/`, `docs/`, root-level files (the core itself) | GPL-3.0-or-later | `LICENSE` |
| `engine/` main body (modified version of Oh My Keymint) | General License Agreement v4 + Oh My Keymint License (**non-commercial use only**; modified versions must carry a modification notice and must not use the original name) | `engine/LICENSE.md`, `engine/LICENSE-2` |
| `engine/soter-ta/` (adapted from ommega) | AGPL-3.0-or-later | `engine/LICENSE-AGPL-3.0.txt` |
| kmr-* crates under `engine/` (from AOSP) | Apache-2.0 | `engine/licenses/Apache-2.0.txt` |
| Local RKP code under `engine/` adapted from Duck-ToolBox / rkp-v1-oracle | MIT | `engine/licenses/` |

The `engine/` binaries may only be used in non-commercial scenarios.

Attribution and modification notices:

- `NOTICE`: the core's notice, plus a summary of where engine/ comes from;
- `engine/NOTICE`: the full modification notice for engine/ (original work, original authors, origin and baseline commit bafe5ba, modifier CharlesGool, changes since 2026-11-01, and the list of modified Apache-2.0 files);
- `engine/docs/THIRD_PARTY.md`: the origin of each third-party component in engine/.

The root-level `NOTICE.engine` is only a placeholder pointing to `engine/NOTICE`.
