# THIRD_PARTY_NOTICES — Third-Party Notices

English · [Español](../es/THIRD_PARTY_NOTICES.md) · [简体中文](../THIRD_PARTY_NOTICES.md)

Third-party materials distributed in this repository, along with their attribution, licensing, and compliance obligations. The core (`patch/`, `docs/`, root-level files) is developed in-house and released under GPL-3.0-or-later. Third-party materials are concentrated in `engine/`.

## Third-Party Notices

### Component List

| Component | Version/Baseline | Upstream | License | Usage | Copyright Holder | Compliance Obligations | Verification Date |
|---|---|---|---|---|---|---|---|
| Oh My Keymint | bafe5ba (1.3.5) | github.com/qwq233/OhMyKeymint, github.com/ITxiao6666/OhMyKeymint | General License Agreement v4 + Oh My Keymint License (non-commercial only) | Main body of `engine/`, renamed and slimmed down | James Clef (qwq233) | Non-commercial use only; include the modification notice (`engine/NOTICE`); must not use the original name | 2026-11-04 |
| AOSP KeyMint (kmr-* crates) | AOSP | android.googlesource.com | Apache-2.0 | `boringssl/`, `common/`, `derive/`, `ta/`, `wire/`, `tests/`, `watchdog/` under `engine/` | The Android Open Source Project | Retain notices; changes are declared file by file in Section 4 of `engine/NOTICE` | 2026-11-04 |
| ommega | — | github.com/Andrea-lyz/ommega, github.com/jiyin004-jpg/ommega | AGPL-3.0-or-later | Adapted in `engine/soter-ta/` | ommega authors | Copyleft; source must be provided | 2026-11-04 |
| Duck-ToolBox | — | github.com/eltavine/Duck-ToolBox | MIT | Adapted in `engine/src/local_rkp/` | Duck-ToolBox authors | Retain notices | 2026-11-04 |
| rkp-v1-oracle | — | github.com/LazyBonesLZY/rkp-v1-oracle | MIT | Adapted in `engine/src/rkp_v1_bridge.rs` | rkp-v1-oracle authors | Retain notices | 2026-11-04 |
| BoringSSL | ed9209cf | boringssl.googlesource.com | See upstream (OpenSSL/ISC, etc.) | Linked at cross-compile time; not distributed with the source | Google | Build dependency; for how to obtain it, see `engine/scripts/build-boringssl.sh` | 2026-11-04 |

### License Texts

Paths to the full license texts in the repository:

- GPL-3.0-or-later (core): `LICENSE`
- General License Agreement v4 + Oh My Keymint License: `engine/LICENSE.md`, `engine/LICENSE-2`
- AGPL-3.0-or-later (soter-ta): `engine/LICENSE-AGPL-3.0.txt`
- Apache-2.0 (kmr-* crates): `engine/licenses/Apache-2.0.txt`
- MIT (Duck-ToolBox, rkp-v1-oracle): `engine/licenses/Duck-ToolBox.txt`, `engine/licenses/rkp-v1-oracle.txt`
- ommega source notice: `engine/licenses/ommega-soter-ta.txt`
- Complete file-by-file modification notice for `engine/`: `engine/NOTICE`; more sources: `engine/docs/THIRD_PARTY.md`

### Source Code Availability

The `keymint` and `inject` binaries produced from `engine/` are subject to the source-availability obligations of GLA v4, AGPL-3.0, and GPL-3.0. The complete corresponding source is the `engine/` directory of this repository. For build instructions, see `engine/README.herald.md` and `engine/scripts/`. BoringSSL is not distributed with this repository; `engine/scripts/build-boringssl.sh` fetches it from upstream at a pinned version.

### Compliance Review

- Copyleft components: GPL-3.0-or-later (core `patch/`), AGPL-3.0-or-later (`engine/soter-ta/`), GLA v4 (main body of `engine/`, non-commercial only).
- Separation analysis: `keymint` and `inject` in `engine/` are standalone binaries launched as init services by the GSI patch `attestation-fix`. They fall under different licenses from the GPL-3.0 `patch/` and do not override each other (see `NOTICE`).
- GLA v4 / Oh My Keymint License is non-commercial only: binaries in `engine/` may only be used in non-commercial settings.
- Review records are in this file and in `engine/NOTICE`.
