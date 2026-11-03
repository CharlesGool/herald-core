# herald-core

[English](docs/en/README.md) · [Español](docs/es/README.md) · 简体中文

[![License](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)](LICENSE)

免 root 核心:一个跑在独立 SELinux 域里的 broker、命令行 `heraldctl` 和插件接口,其他补丁按接口接入。**产物是 GSI 补丁 `herald-core`**(`make dist` → `dist/herald-core/`),用在 TB321FU-GSI-Mods 里。

文档:[设计](docs/DESIGN.md) · [插件接口](docs/plugin-api.md) · [变更](docs/CHANGELOG.md) · [维护日志](docs/LOG.md) · [历史](docs/HISTORY.md) · [第三方声明](docs/THIRD_PARTY_NOTICES.md)

## 简介

| 目录 | 内容 |
|---|---|
| `patch/` | 补丁源码,详见 `patch/README.md` |
| `docs/plugin-api.md` | 插件接口说明 |
| `engine/` | 认证引擎(HeraldKeymint)源码,fork 自 Oh My Keymint,标识已改 herald,只保留编 `keymint` 和 `inject` 需要的部分;`make engine` 编译,说明见 `engine/README.herald.md` |
| `engine/src/` | keymint 守护(替代 keystore2 后端的 KeyMint 实现、keybox、本地 RKP) |
| `engine/injector/` | inject 注入器(ptrace 注入 keystore2,按 App 路由请求) |
| `engine/common/`、`boringssl/`、`ta/`、`wire/`、`derive/`、`watchdog/`、`tests/` | AOSP KeyMint 的 kmr-* crate 等基础库 |
| `engine/soter-ta/`、`pif-common/` | 高通 Soter 软件 TA(改编自 ommega)和共用的 Soter 定义 |
| `engine/aidl/`、`proto/`、`runtime/`、`assets/` | AIDL 接口、protobuf、RKP V1 bridge 的 Java 部分、编进二进制的资源文件 |
| `engine/scripts/` | `build-android.sh`(`make engine` 调用)、`build-boringssl.sh`(按固定版本编 BoringSSL) |
| `engine/docs/` | 上游文档(配置说明、FAQ、第三方来源);模块模式专用的章节已标注 |

上游里只服务于 Magisk/KernelSU 模块模式的部分(`webui/`、`template/`、`pif-spoof/`、`hal/`、打包脚本等)已经删掉,原因见 `engine/README.herald.md`。

**和 TB321FU-GSI-Mods 的关系**:这里是唯一来源。Mods 里的 `patches/required/general/herald-core` 是同步副本,改完这里后在 Mods 跑 `make sync-core`(`make unit` 会检查两边一致)。整体设计见 [docs/DESIGN.md](docs/DESIGN.md)。

## 要求

- 编引擎(`make engine`):Linux 构建主机,Android NDK、BoringSSL、Rust nightly 工具链,见 `engine/README.herald.md` 和 `.env.example`。
- 只用补丁(`make dist`):无额外要求。

## 安装

无需独立安装。作为 GSI 补丁,由 TB321FU-GSI-Mods 在构建时打入 system 镜像(`make sync-core` 同步本仓库的 `patch/`)。

## 指南

```bash
make dist      # 生成 GSI 补丁到 dist/herald-core/
make engine    # 交叉编译认证引擎 keymint、inject(需工具链)
```

设备上命令行用法见 `patch/README.md`(`heraldctl selftest` / `diag` / `pair` 等);插件开发见 [docs/plugin-api.md](docs/plugin-api.md)。

## 卸载

不单独安装;从 TB321FU-GSI-Mods 的构建选择里去掉 `herald-core` 补丁重新构建即可。

## 致谢

本项目的认证引擎基于 [Oh My Keymint](https://github.com/qwq233/OhMyKeymint) 等开源项目,完整署名、来源和许可见 [docs/THIRD_PARTY_NOTICES.md](docs/THIRD_PARTY_NOTICES.md) 与 `engine/NOTICE`。

## 许可证

本仓库多许可并存,各管各的部分,互不覆盖:

| 部分 | 许可证 | SPDX | 全文 |
|---|---|---|---|
| `patch/`、`docs/`、根目录文件(核心本身) | GNU GPL v3.0 或更新 | `GPL-3.0-or-later` | `LICENSE` |
| `engine/` 主体(Oh My Keymint 的修改版) | General License Agreement v4 + Oh My Keymint License(**仅限非商业使用**;修改版须附修改声明、不得使用原名) | — | `engine/LICENSE.md`、`engine/LICENSE-2` |
| `engine/soter-ta/`(改编自 ommega) | GNU AGPL v3.0 或更新 | `AGPL-3.0-or-later` | `engine/LICENSE-AGPL-3.0.txt` |
| `engine/` 下的 kmr-* crate(来自 AOSP) | Apache License 2.0 | `Apache-2.0` | `engine/licenses/Apache-2.0.txt` |
| `engine/` 里改编自 Duck-ToolBox / rkp-v1-oracle 的本地 RKP 代码 | MIT | `MIT` | `engine/licenses/` |

`engine/` 的二进制只能在非商业场景使用。署名和修改声明见 `NOTICE`、`engine/NOTICE`、[docs/THIRD_PARTY_NOTICES.md](docs/THIRD_PARTY_NOTICES.md);根目录的 `NOTICE.engine` 只是指向 `engine/NOTICE` 的占位。
