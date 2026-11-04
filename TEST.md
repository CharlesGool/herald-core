# herald P2.0 真机测试:注入 system_server 的可行性(应用列表隐藏第一步)

这轮**只验一件事**:把注入器坐进 `system_server`、挂上 binder hook 之后,**系统还能不能正常跑、三绿会不会受影响**。还没有任何过滤逻辑(P2.0 的扩展是**纯透传**,每个事务原样放行)。

**先刷 `..._test-v1.3.6_optional/` 这个包**(比上一个多了一个 shell 可读的结果标记——这台设备 logcat 被关、root 日志读不到,只能靠这个文件看注入到底成没成)。

**安全设计**:system_server 注入**默认关**,靠一次性标志触发;脚本开机**先删标志再注入**,所以就算把 system_server 搞崩也只崩这一次开机、下次自动恢复,**不会 bootloop**;注入失败**绝不 kill system_server**。

环境:Windows cmd / PowerShell。引号里的管道是手机上跑的,没问题。

---

## 第 1 轮:默认启动(注入**关**),确认没搞坏 keystore 认证

刷入 v1.3.6,正常开机。确认和之前一致:
```bat
adb shell heraldctl selftest
```
```bat
adb shell heraldctl keybox state
```
期望:selftest 绿、keybox `valid / source=google_remote / level=tee`,Play Integrity 三绿。

## 第 2 轮:打开 system_server 注入(一次性),看结果 + 稳不稳

```bat
adb shell touch /data/local/tmp/herald_ss_inject
```
```bat
adb reboot
```

开机后**等约 30 秒**(注入在开机后延迟 ~8 秒才做),然后读结果标记:

```bat
adb shell cat /data/local/tmp/herald_ss_result
```

这一条就是关键,对照:
- `ok system_server 注入返回成功 pid=... rc=0` → **注进去了**。而且你此刻能正常读文件/跑命令 = system_server 没崩、设备稳 → **P2.0 基本通过**。
- `fail system_server 注入未成功 rc=...` → 启动器跑了但没注进去(system_server 没被动,系统应照常)。把这行发我。
- `attempting ...`(等够 30 秒还是它)→ 注入卡住了(可能 ptrace 卡住了 system_server)。看设备是不是发卡;把这行发我。
- **`No such file or directory`(文件不存在)** → 启动器**没跑到**:要么没刷 v1.3.6,要么 `/data/local/tmp` 开机被清了(标志在启动器读之前就没了)。告诉我,我改标志的存法。

然后照常回归:
```bat
adb shell heraldctl selftest
```
```bat
adb shell heraldctl keybox state
```
Play Integrity 再跑一次——应仍三绿(keystore 和 system_server 是两条独立注入)。

最后**用几分钟**:开几个 app、下拉通知、进设置,看有没有卡顿/ANR/闪退(system_server 被 hook 后所有 binder 调用都过它,这步看透传开销可不可接受)。

---

## 判定

- 标记 `ok` + 设备稳 + 三绿不变 + 不卡 → **P2.0 通过**,下一步 P2.1 做真正的 `getInstalledPackages` 过滤。
- 标记 `fail` / `attempting` 卡住 / bootloop / 明显卡顿 → 注入 system_server 当前方式有问题。设备会自动恢复(标志一次性,已消费),把标记内容 + 现象发我定位。

## 出问题怎么发回
```bat
adb shell cat /data/local/tmp/herald_ss_result
```
```bat
adb shell heraldctl selftest > selftest.txt 2>&1
```
```bat
adb shell "ps -A | grep -w system_server" > ps.txt 2>&1
```
说明:标记内容、设备稳不稳、第 1 轮是否正常。
