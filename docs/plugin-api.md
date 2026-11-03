# 免 root 核心(herald-core)和接入它的补丁

把原来靠 KernelSU/Magisk + Zygisk 才能做的事,改成系统自带的 init 服务:一个跑在自己 SELinux 域(`herald`)里的 root broker,加一个无特权的命令行 `heraldctl`,供前端 App 或 adb 控制。
核心**只提供基础能力和接入方式**(命令通道、配对、工具函数、插件接口),具体功能都是别的补丁接进来的插件。

| 补丁 | 做什么 | 运行时开关 |
|---|---|---|
| [herald-core](https://github.com/CharlesGool/TB321FU-GSI-Mods/tree/main/patches/required/general/herald-core) | 核心:herald 域 + broker + heraldctl + 插件接口 + `bin/resetprop` | `heraldctl selftest|pair|diag|log|raw|reboot` |
| [attestation-fix](https://github.com/CharlesGool/TB321FU-GSI-Mods/tree/main/patches/required/general/attestation-fix) | keymint + inject:伪造 keystore2 的认证链;指纹/keybox/RKP/安全补丁命令 | `heraldctl spoof|keybox|rkp` |
| 默认身份(原 spoof/default-profile,已并入 [attestation-fix](https://github.com/CharlesGool/TB321FU-GSI-Mods/blob/main/patches/required/general/attestation-fix/README.md)) | 引擎的默认 profile 用本机真实身份(不再是烤进去的 Pixel 7);只影响新装/清数据后的默认状态 | 无 |
| [spoof/global-pif](https://github.com/CharlesGool/TB321FU-GSI-Mods/tree/main/patches/optional/spoof/global-pif) | 全局指纹伪装(post-fs-data 用 resetprop 覆写 build 指纹) | `heraldctl spoof set|off`,默认关 |
| [spoof/app-spoof](https://github.com/CharlesGool/TB321FU-GSI-Mods/tree/main/patches/optional/spoof/app-spoof) | 按 App 伪装 Build.* | `heraldctl spoofmode|spoofapps` |
| [herald-readonly-shell](https://github.com/CharlesGool/TB321FU-GSI-Mods/tree/main/patches/debug/herald-readonly-shell) | `rexec`:以 root 跑白名单里的只读查看命令 | `heraldctl rexec on [分钟]|off`,默认关,到点自动关 |
| [hotspot-control/002.control](https://github.com/CharlesGool/TB321FU-GSI-Mods/tree/main/patches/optional/hotspot-control/parts/002.control) | Wi-Fi 热点覆盖项读写、热点开关 | `heraldctl wifi|hotspot` |
| battery、perf、game-mode、refresh-rate、navbar-hide、statusbar-netspeed、devopts-hide、custom-font、app-backup、tango… | 各自的运行时命令 | 各自的 `heraldctl` 子命令 |

依赖关系:所有往核心接入的补丁都 `REQUIRES herald-core`(Wi-Fi 热点命令是软依赖 `WANTS`);伪装类补丁再 `REQUIRES attestation-fix`。

## 插件接口

接口(broker 插件的 `heraldp_cmd_*` / `heraldp_cmdf_*` / `HERALDP_START` / `HERALDP_IDLE` / `HERALDP_ATTACH` / `HERALDP_SELFTEST` / `HERALDP_DIAG`,heraldctl 插件的 `heraldctl_cmd_*` / `HERALDCTL_HELP`)的完整说明在 [herald-core/README.md](https://github.com/CharlesGool/TB321FU-GSI-Mods/blob/main/patches/required/general/herald-core/README.md)。

## 整组独立发布

引擎(keymint / inject)从本仓库 `engine/` 的源码编译(`make engine`,见 `engine/README.herald.md`),不再从别的模块里提取。
keybox 和 `config.toml` 里的本机私密种子仍然是**私有**的,不进仓库;它们放在 Mods 仓库设备配置(`devices/<设备>/conf.d/herald.conf`)里 `HERALD_PAYLOAD` 指向的私有目录里,由用户自己准备。
`bin/resetprop` 同样放在 `HERALD_PAYLOAD` 里,不进仓库,说明见 Mods 仓库 [herald-core README 的 HERALD_PAYLOAD 一节](https://github.com/CharlesGool/TB321FU-GSI-Mods/blob/main/patches/required/general/herald-core/README.md#输入deviceconf);二进制的具体出处(哪个项目、哪个版本)**待补**。

后端接口(命令、回包格式、配对认证)的文档是 `herald-backend-api.md`,给前端 App 开发用。它在 `tb321fu_pixelos_15_tangorpro_ap3a.241005.015/doc/` 下(旧工程目录),**不在本仓库里**,也不在 Mods 仓库里。
