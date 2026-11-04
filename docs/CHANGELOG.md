# CHANGELOG — 变更记录

[English](en/CHANGELOG.md) · [Español](es/CHANGELOG.md) · 简体中文

## V1.5 — 2026-11-04

### 移除
- 删除 SOTER(高通/腾讯)全部实现:injector `hook/soter.rs`/`soter_watch.rs`、keymint `soter_beta.rs`/`soter_hal.rs`、`soter-ta` crate、pif-common `soter.rs`/`soter_blob.rs`(TB321FU 不启用)。
- 去掉随之的 AGPL-3.0:AGPL 只因 ommega 的 soter-ta 引入,删 `engine/LICENSE-AGPL-3.0.txt`、`licenses/ommega-soter-ta.txt`,NOTICE/THIRD_PARTY_NOTICES/README 相应更新。

### 修复
- resetprop 对已删 `--soter-watch` 的悬空调用:原会每 60 秒重新注入 keystore2(删 SOTER 后暴露的 HIGH 缺陷)。

### 变更
- 注入器加 `inject --target keystore2|system_server` 目标选择,`entry` 按进程名选扩展;keystore 认证路径逐字节不变,HERALD RPC 会话只在 keystore 分支建立。
- 新增透传的 `PkgManagerInterceptor`(P2.0,默认不触发),为应用列表隐藏做底层验证。

### 评审
- P2(注入 system_server 拦 `IPackageManager` 隐藏应用列表)经真机 de-risk 被 SELinux 堵死(system_server enforcing 无法加载 `herald_exec` 注入体,放行会撞 neverallow/削弱安全),**搁置**;改走 `services.jar`。详见 `docs/injector-capability-design.md`。

## V1.4 — 2026-11-04

### 新增
- broker 插件接口加 `HERALDP_ASYNC`:插件可声明慢命令后台执行,不阻塞主循环(可选,默认空集则行为不变)。

## V1.3 — 2026-11-04

### 修复
- broker 工作目录移回 `/data/local/tmp`,修复 V1.2 回包为空的真机回归(SELinux 标签)。

## V1.2 — 2026-11-03

### 变更
- engine 安全加固:未知 Android 版本不再 panic、`keymint.dat` 原子写、keybox 解析失败不覆盖磁盘文件。

## V1.1 — 2026-11-03

### 变更
- broker 自适应轮询(连续操作几乎不等待,空闲回落)。
- 核心 shell 加固:回包/配对 token 写入加 O_EXCL、Settings 键名校验、`heraldctl log` 经 broker 读。

## V1.0 — 2026-11-01

### 新增
- 首个版本:broker、`heraldctl`、插件接口,以及 fork 自 Oh My Keymint 的认证引擎(keymint、inject)。
