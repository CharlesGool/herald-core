# THIRD_PARTY_NOTICES — 第三方声明

[English](en/THIRD_PARTY_NOTICES.md) · [Español](es/THIRD_PARTY_NOTICES.md) · 简体中文

本仓库分发的第三方材料及其署名、许可与合规义务。核心(`patch/`、`docs/`、根目录文件)为自研,以 GPL-3.0-or-later 发布;第三方材料集中在 `engine/`。

## 第三方声明

### 组件清单

| 组件 | 版本/基线 | 上游 | 许可证 | 使用方式 | 版权归属 | 合规义务 | 核验日期 |
|---|---|---|---|---|---|---|---|
| Oh My Keymint | bafe5ba(1.3.5) | github.com/qwq233/OhMyKeymint、github.com/ITxiao6666/OhMyKeymint | General License Agreement v4 + Oh My Keymint License(仅非商业) | `engine/` 主体,改名并精简改造 | James Clef (qwq233) | 非商业使用;附修改声明(`engine/NOTICE`);不得使用原名 | 2026-11-04 |
| AOSP KeyMint(kmr-* crate) | AOSP | android.googlesource.com | Apache-2.0 | `engine/` 的 `boringssl/`、`common/`、`derive/`、`ta/`、`wire/`、`tests/`、`watchdog/` | The Android Open Source Project | 保留声明;改动已在 `engine/NOTICE` 第四节逐文件声明 | 2026-11-04 |
| Duck-ToolBox | — | github.com/eltavine/Duck-ToolBox | MIT | `engine/src/local_rkp/` 改编 | Duck-ToolBox 作者 | 保留声明 | 2026-11-04 |
| rkp-v1-oracle | — | github.com/LazyBonesLZY/rkp-v1-oracle | MIT | `engine/src/rkp_v1_bridge.rs` 改编 | rkp-v1-oracle 作者 | 保留声明 | 2026-11-04 |
| BoringSSL | ed9209cf | boringssl.googlesource.com | 见上游(OpenSSL/ISC 等) | 交叉编译时链接,不随源码分发 | Google | 构建依赖;获取方式见 `engine/scripts/build-boringssl.sh` | 2026-11-04 |

### 许可证文本

完整原文在仓库中的路径:

- GPL-3.0-or-later(核心):`LICENSE`
- General License Agreement v4 + Oh My Keymint License:`engine/LICENSE.md`、`engine/LICENSE-2`
- Apache-2.0(kmr-* crate):`engine/licenses/Apache-2.0.txt`
- MIT(Duck-ToolBox、rkp-v1-oracle):`engine/licenses/Duck-ToolBox.txt`、`engine/licenses/rkp-v1-oracle.txt`
- `engine/` 的完整逐文件修改声明:`engine/NOTICE`;更多来源:`engine/docs/THIRD_PARTY.md`

### 源代码提供

`engine/` 产出的 `keymint`、`inject` 二进制受 GLA v4 和 GPL-3.0 的源码提供义务约束。完整对应源码即本仓库 `engine/` 目录;构建方式见 `engine/README.herald.md` 和 `engine/scripts/`。BoringSSL 不随本仓库分发,由 `engine/scripts/build-boringssl.sh` 按固定版本从上游获取。

### 合规审查

- Copyleft 组件:GPL-3.0-or-later(核心 `patch/`)、GLA v4(`engine/` 主体,限非商业)。
- 分离分析:`engine/` 的 `keymint`/`inject` 作为独立二进制,由 GSI 补丁 `attestation-fix` 以 init 服务方式拉起,与 GPL-3.0 的 `patch/` 分属不同许可、互不覆盖(见 `NOTICE`)。
- GLA v4 / Oh My Keymint License 限非商业:`engine/` 的二进制只能在非商业场景使用。
- 审查记录见本文件与 `engine/NOTICE`。
