# DESIGN — 设计

[English](en/DESIGN.md) · [Español](es/DESIGN.md) · 简体中文

## 设计目标

- 提供"不依赖 root 工具也能让系统服务以 root 代执行"的基础能力和接入方式,核心自身不带任何具体功能。
- 命令通道对调用方无特权要求(命令行 `heraldctl`;配对后的前端 App),按白名单代执行。
- 认证引擎从 keystore2 后端接管 KeyMint,提供设备密钥认证,作为插件接入,而非核心内置。
- 与底包解耦:核心是 shell 脚本加一个 `resetprop`,换 GSI 不用改。

非目标:核心不实现伪装、电池、性能等具体功能,这些由各功能补丁作为插件提供。

## 架构

| 模块 | 职责 |
|---|---|
| broker(`patch/files/herald-start.sh`) | 跑在独立 SELinux 域 `herald`,boot_completed 后起;轮询请求文件,按白名单代执行 |
| `heraldctl`(`patch/files/heraldctl`) | 无特权命令行前端,经命令通道把请求交给 broker |
| 插件接口 | 其他补丁往插件目录放文件,给 broker 和 heraldctl 加命令、守护、自检 |
| 认证引擎(`engine/`,HeraldKeymint) | `keymint`(KeyMint 守护、keybox、本地 RKP)和 `inject`(ptrace 注入 keystore2);fork 自 Oh My Keymint |

两条命令通道:命令行(`/data/local/tmp/herald`,shell 可写,只放 req/resp)和配对的前端 App(设备随机密钥 + uid 校验)。回包先写临时文件再 rename,避免读到半截。

顶层目录职责:

- `patch/` — 核心的 GSI 补丁源码(broker、heraldctl、sepolicy、init);产物 `make dist` → `dist/herald-core/`。
- `engine/` — 认证引擎源码(fork,见 `engine/README.herald.md`)。
- `docs/` — 项目文档。

## 设计约束

- 插件接口(broker 的 `heraldp_*`、变量;heraldctl 的 `send`/`heraldctl_cmd_*`)是其他补丁依赖的契约,改动须保持兼容。
- 回包的工作目录必须在 `/data/local/tmp`(shell 域 enforcing 才读得到带 `shell_data_file` 标签的回包);放 `/data/adb` 会因 SELinux 标签不符导致回包读不出(V1.2 真机实测)。
- broker 域 permissive,但通道目录只放 req/resp,日志与工作目录隔离,避免符号链接劫持 root 写入。

## 外部接口

前端 App 通过配对通道(`files/herald_req`,随机密钥 + uid 校验)收发命令,协议见项目后端接口文档。命令行通道面向 uid 0/2000。

## 扩展

插件机制见 `docs/plugin-api.md`:broker 插件(`heraldp_cmd_*` 等)和 heraldctl 插件(`heraldctl_cmd_*`),以及 payload 目录标签、SELinux 规则追加的约定。
