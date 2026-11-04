# LOG — 维护日志

[English](en/LOG.md) · [Español](es/LOG.md) · 简体中文

## 记录

版本变更见 [CHANGELOG.md](CHANGELOG.md),历史需求与旧交接见 [HISTORY.md](HISTORY.md)。

## 缺陷

暂无。

## 限制

- `engine/` 的 `keymint`/`inject` 二进制受 GLA v4 约束,仅限非商业使用。

## 决策

| 日期 | 决策 | 状态 | 原因 |
|---|---|---|---|
| 2026-11-01 | engine 全量改名 omk→herald | 接受 | Oh My Keymint License 第 3 条不允许超范围使用原名 |
| 2026-11-04 | broker 工作目录留在 `/data/local/tmp` | 接受 | 移到 `/data/adb` 会因 SELinux 标签导致回包读不出(V1.2 真机回归) |
| 2026-11-04 | 删除 SOTER 并去掉随之的 AGPL | 接受 | TB321FU 不启用 SOTER;AGPL 仅因 ommega soter-ta 引入,一并去除 |
| 2026-11-04 | 应用列表隐藏不走注入 system_server | 接受 | system_server(enforcing)无法加载 herald_exec 注入体,放行会撞 neverallow/削弱安全;改走 services.jar(真机 de-risk 证实) |

## 交接

未发现进行中的工作。当前分支 `main`,与 `origin/main` 同步。
