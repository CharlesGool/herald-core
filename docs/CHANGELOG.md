# CHANGELOG — 变更记录

[English](en/CHANGELOG.md) · [Español](es/CHANGELOG.md) · 简体中文

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
