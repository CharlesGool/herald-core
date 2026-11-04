# HISTORY — 历史

[English](en/HISTORY.md) · [Español](es/HISTORY.md) · 简体中文

- 2026-11-01:项目建立。核心(broker + heraldctl + 插件接口)自研,认证引擎由 Oh My Keymint(基线 bafe5ba)fork 而来并改名为 herald。原名 `omk-core`,后改为 `herald-core`。
- 2026-11-04:工作目录真机回归修复后,V1.3 随 TB321FU 的 `user-v1.3` 正式版存档。
- 2026-11-04:V1.5 删除 SOTER(连带去掉 AGPL)、修复 resetprop 悬空 `--soter-watch`(HIGH);注入器能力化加 P2.0 目标选择/透传。P2(注入 system_server 隐藏应用列表)经真机验证被 SELinux 堵死,搁置并改走 services.jar。随 TB321FU `user-v1.4` 锚点正式版存档。
