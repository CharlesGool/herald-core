# herald-core 的认证引擎(engine/)

这是 [herald-core](https://github.com/CharlesGool/herald-core) 的认证引擎(keymint 守护 + inject 注入器),不是核心本身。
它 fork 自 Oh My Keymint(上游 [ITxiao6666/OhMyKeymint](https://github.com/ITxiao6666/OhMyKeymint),基线提交 bafe5ba,原作者 James Clef / qwq233),标识全部改成了 herald / HeraldKeymint。
编出来的 `keymint` 和 `inject` 由 GSI 补丁 `attestation-fix` 打进系统,在它的 `HERALDP_START` 里拉起。

- 来源、署名和修改声明:见本目录的 [`NOTICE`](NOTICE)。
- 原 README 保留为 [`README.md`](README.md)(上游原文,里面的 OMK 指原项目,很多内容只适用于 Magisk/KernelSU 模块模式)。

## 许可

本目录有几种许可并存,以各文件和 crate 的声明为准:

| 部分 | 许可 | 全文 |
|---|---|---|
| 引擎主体(原作者拥有著作权的部分) | General License Agreement v4 + Oh My Keymint License(**仅限非商业使用**,修改版须附修改声明、不得使用原名) | [`LICENSE.md`](LICENSE.md)、[`LICENSE-2`](LICENSE-2) |
| `soter-ta/`(改编自 ommega) | AGPL-3.0-or-later | [`LICENSE-AGPL-3.0.txt`](LICENSE-AGPL-3.0.txt) |
| kmr-* crate(`boringssl/`、`common/`、`derive/`、`ta/`、`wire/`、`tests/`)和 `watchdog/`,来自 AOSP | Apache-2.0 | [`licenses/Apache-2.0.txt`](licenses/Apache-2.0.txt) |
| `src/local_rkp/` 等改编自 Duck-ToolBox / rkp-v1-oracle 的部分 | MIT | [`licenses/`](licenses/) |

注意:上游 `README.md` 的"许可与致谢"一节写的是"同意 [AGPL-3.0-or-later](LICENSE.md)",但 `LICENSE.md` 实际是 GLA v4,不是 AGPL。实际情况以上表为准;AGPL 全文是另外放的 `LICENSE-AGPL-3.0.txt`。

## 构建

```sh
make engine          # 在 herald-core 根目录运行
```

`make engine` 底层调用 `engine/scripts/build-android.sh`,产物是 `keymint` 和 `inject`(aarch64-linux-android,release)。
BoringSSL 由 `engine/scripts/build-boringssl.sh` 按固定版本编译。

依赖:

| 依赖 | 版本 |
|---|---|
| Rust | nightly,具体版本由 `rust-toolchain.toml` 固定 |
| Android NDK | 29.0.14206865 |
| BoringSSL | ed9209cff8b5a2fbc6c10050f29b3355048bda85 |
| Android build-tools | 35(用 d8 编 RKP V1 bridge 的 Java 部分) |
| JDK | 17 |
| protoc | 系统包即可 |

## 相对上游删掉的部分

herald 只用 `keymint` 和 `inject` 两个二进制,而且由 init 服务拉起,不装成 Magisk/KernelSU 模块,所以下列只服务于模块模式或其他功能的部分已经删掉:

| 删掉的 | 原用途 |
|---|---|
| `hal/` | AOSP kmr-hal 的源码(HAL 服务适配层);上游里就没有 Cargo.toml、不在 workspace 里,不参与编译 |
| `pif-spoof/` | Zygisk 注入的 PIF 指纹伪装和腾讯 Soter Beta;herald 的伪装走 GSI 补丁 spoof/* |
| `third_party/zygisk-api/` | Zygisk 头文件,只给 pif-spoof 用 |
| `template/` | 模块模板(module.prop、customize.sh、post-fs-data.sh、service.sh、WebUI 产物等);被代码 `include_str!` 的文件已挪到 `assets/`,第三方许可文本挪到 `licenses/` |
| `webui/` | 模块的 WebUI;herald 的前端是单独的 App,经 heraldctl 控制 |
| `build.py`、模块打包脚本、`update.json` | 打 Magisk/KSU 模块 zip 和模块在线更新 |
| `.github/`、`.cargo-husky/` | 上游的 CI 和提交钩子 |

所以 `docs/` 里讲模块安装、WebUI、模块目录的章节对 herald 不适用,已在章节开头标注。
