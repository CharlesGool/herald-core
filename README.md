# herald-core

免 root 核心:一个跑在独立 SELinux 域里的 broker、命令行 `heraldctl` 和插件接口,其他补丁按接口接入。
**产物是 GSI 补丁 `herald-core`**(`make dist` → `dist/herald-core/`),用在 TB321FU-GSI-Mods 里。

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

**和 TB321FU-GSI-Mods 的关系**:这里是唯一来源。Mods 里的 `patches/required/general/herald-core` 是同步副本,改完这里后在 Mods 跑 `make sync-core`(`make unit` 会检查两边一致)。

## 许可

本仓库里几种许可并存,各管各的部分,互不覆盖:

| 部分 | 许可证 | 全文 |
|---|---|---|
| `patch/`、`docs/`、根目录文件(核心本身) | GPL-3.0-or-later | `LICENSE` |
| `engine/` 主体(Oh My Keymint 的修改版) | General License Agreement v4 + Oh My Keymint License(**仅限非商业使用**;修改版须附修改声明、不得使用原名) | `engine/LICENSE.md`、`engine/LICENSE-2` |
| `engine/soter-ta/`(改编自 ommega) | AGPL-3.0-or-later | `engine/LICENSE-AGPL-3.0.txt` |
| `engine/` 下的 kmr-* crate(来自 AOSP) | Apache-2.0 | `engine/licenses/Apache-2.0.txt` |
| `engine/` 里改编自 Duck-ToolBox / rkp-v1-oracle 的本地 RKP 代码 | MIT | `engine/licenses/` |

`engine/` 的二进制只能在非商业场景使用。

署名和修改声明:

- `NOTICE`:核心的说明,以及 engine/ 的来源概要;
- `engine/NOTICE`:engine/ 的完整修改声明(原作品、原作者、来源和基线提交 bafe5ba、修改人 CharlesGool、2026-11-01 起的修改内容,以及改过的 Apache-2.0 文件清单);
- `engine/docs/THIRD_PARTY.md`:engine/ 里各第三方组件的来源。

根目录的 `NOTICE.engine` 只是一个指向 `engine/NOTICE` 的占位。
