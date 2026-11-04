# herald-core

English · [Español](../es/README.md) · [简体中文](../../README.md)

[![License](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)](../../LICENSE)

Rootless core: a broker running in its own SELinux domain, the `heraldctl` command-line tool, and a plugin interface that other patches connect to through the interface. **The build artifact is the GSI patch `herald-core`** (`make dist` → `dist/herald-core/`), used in TB321FU-GSI-Mods.

Docs: [Design](DESIGN.md) · [Plugin API](../plugin-api.md) · [Changelog](CHANGELOG.md) · [Maintenance log](LOG.md) · [History](HISTORY.md) · [Third-party notices](THIRD_PARTY_NOTICES.md)

## Overview

| Directory | Contents |
|---|---|
| `patch/` | Patch source; see `patch/README.md` for details |
| `docs/plugin-api.md` | Plugin interface description |
| `engine/` | Authentication engine (HeraldKeymint) source, forked from Oh My Keymint with identifiers changed to herald. Only the parts needed to build `keymint` and `inject` are kept. Build with `make engine`; see `engine/README.herald.md` |
| `engine/src/` | keymint daemon (KeyMint implementation replacing the keystore2 backend, keybox, local RKP) |
| `engine/injector/` | inject injector (injects into keystore2 via ptrace, routes requests per app) |
| `engine/common/`, `boringssl/`, `ta/`, `wire/`, `derive/`, `watchdog/`, `tests/` | Base libraries such as the kmr-* crates from AOSP KeyMint |
| `engine/pif-common/` | Shared PIF (fingerprint profile) definitions |
| `engine/aidl/`, `proto/`, `runtime/`, `assets/` | AIDL interfaces, protobuf, the Java part of the RKP V1 bridge, and resource files compiled into the binary |
| `engine/scripts/` | `build-android.sh` (called by `make engine`), `build-boringssl.sh` (builds BoringSSL at a pinned version) |
| `engine/docs/` | Upstream docs (configuration notes, FAQ, third-party sources); sections specific to module mode are marked |

The parts of upstream that only serve Magisk/KernelSU module mode (`webui/`, `template/`, `pif-spoof/`, `hal/`, packaging scripts, etc.) have been removed. The reasons are in `engine/README.herald.md`.

**Relationship with TB321FU-GSI-Mods**: this repository is the single source of truth. `patches/required/general/herald-core` in Mods is a synced copy. After changing things here, run `make sync-core` in Mods (`make unit` checks that the two copies match). For the overall design, see [docs/DESIGN.md](DESIGN.md).

## Requirements

- Building the engine (`make engine`): a Linux build host, Android NDK, BoringSSL, and the Rust nightly toolchain; see `engine/README.herald.md` and `.env.example`.
- Patch only (`make dist`): no additional requirements.

## Installation

No separate installation is needed. As a GSI patch, it is built into the system image by TB321FU-GSI-Mods (`make sync-core` syncs this repository's `patch/`).

## Usage

```bash
make dist      # Generate the GSI patch into dist/herald-core/
make engine    # Cross-compile the authentication engine keymint and inject (requires toolchain)
```

For on-device command-line usage, see `patch/README.md` (`heraldctl selftest` / `diag` / `pair`, etc.); for plugin development, see [docs/plugin-api.md](../plugin-api.md).

## Uninstall

There is no separate installation to remove. Deselect the `herald-core` patch in the TB321FU-GSI-Mods build options and rebuild.

## Acknowledgements

The authentication engine of this project is based on open-source projects including [Oh My Keymint](https://github.com/qwq233/OhMyKeymint). Full attribution, sources, and licenses are in [docs/THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) and `engine/NOTICE`.

## License

This repository uses multiple licenses. Each covers its own part, and they do not override one another:

| Part | License | SPDX | Full text |
|---|---|---|---|
| `patch/`, `docs/`, root-level files (the core itself) | GNU GPL v3.0 or later | `GPL-3.0-or-later` | `LICENSE` |
| Main body of `engine/` (modified version of Oh My Keymint) | General License Agreement v4 + Oh My Keymint License (**non-commercial use only**; modified versions must carry a modification notice and must not use the original name) | — | `engine/LICENSE.md`, `engine/LICENSE-2` |
| kmr-* crates under `engine/` (from AOSP) | Apache License 2.0 | `Apache-2.0` | `engine/licenses/Apache-2.0.txt` |
| Local RKP code under `engine/` adapted from Duck-ToolBox / rkp-v1-oracle | MIT | `MIT` | `engine/licenses/` |

Binaries from `engine/` may only be used in non-commercial settings. Attribution and modification notices are in `NOTICE`, `engine/NOTICE`, and [docs/THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md); the root-level `NOTICE.engine` is only a placeholder pointing to `engine/NOTICE`.
