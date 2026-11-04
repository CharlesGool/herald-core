# herald P2.0 真机测试:注入 system_server 的可行性(应用列表隐藏第一步)

这轮**只验一件事**:把注入器坐进 `system_server`、挂上 binder hook 之后,**系统还能不能正常跑、三绿会不会受影响**。还没有任何过滤逻辑(P2.0 的扩展是**纯透传**,每个事务原样放行)。过滤 app 列表是下一步 P2.1。

**安全设计(重要)**:system_server 注入**默认关**,靠一次性标志触发;脚本开机时见到标志会**先删掉再注入**——所以即便这次把 system_server 搞崩重启,下次开机已无标志、不再注入,**自动恢复,不会 bootloop**。注入失败也**绝不 kill system_server**。

环境:Windows cmd / PowerShell,adb 已连上。Windows 这边别用管道 `|`;引号里的管道是在手机上跑的,没问题。

---

## 第 1 轮:默认启动(注入**关**),先确认新 inject 没搞坏 keystore 认证

刷入新包,正常开机(system_server 注入默认不发生)。确认和之前一致:

```bat
adb shell heraldctl selftest
```
```bat
adb shell heraldctl keybox state
```
期望:selftest 绿(`uid=0 / ctx=u:r:herald:s0 / selinux=Enforcing / keymint=present`),keybox `valid / source=google_remote / level=tee`。再用 Play Integrity 检测器确认**三绿**(伪装之前开过就还在)。

> 这一轮等于回归:新 inject 二进制加了 system_server 支持,但默认只注入 keystore2,keystore 路径逐字节没变——三绿必须和上个包一样。

## 第 2 轮:打开 system_server 注入(一次性),验系统是否稳

设一次性标志,然后重启:

```bat
adb shell touch /data/local/tmp/herald_ss_inject
```
```bat
adb reboot
```

开机后**等约 20~30 秒**(注入在开机后延迟 ~8 秒才做),然后:

**① 最重要——设备正常吗?**
- 能正常进桌面、UI 不卡死、没有反复重启。这就是 P2.0 的核心结论:注入 system_server 是否可行。

**② 看注入结果(走 broker 日志,root 私有日志 adb 读不到)**
```bat
adb shell heraldctl log
```
找这几行之一:
- `注入 system_server pid=...` + `✓ system_server 注入返回成功` → 注入成功。
- `✗ system_server 注入未成功 ...` → 没注进去(但 system_server 没被动,系统应照常)。
- 没有任何 `system_server` 行 → 标志没被读到(确认标志路径、是否真重启了)。

**③ system_server 正常单实例**
```bat
adb shell "ps -A | grep -w system_server"
```
期望:就一个 `system_server`,没有在反复重启(隔半分钟再看 PID 不变)。

**④ 三绿没被连累**(keystore 认证和 system_server 实验是两条独立注入)
```bat
adb shell heraldctl selftest
```
```bat
adb shell heraldctl keybox state
```
再跑一次 Play Integrity。期望:仍三绿。

**⑤ 用几分钟**:随便开几个 app、下拉通知、进设置,确认没有卡顿/ANR/闪退(system_server 被 hook 后所有 binder 调用都经过它,这步是看透传开销可不可接受)。

---

## 判定

- 第 2 轮**设备稳定 + 注入返回成功 + 三绿不变 + 无明显卡顿** → **P2.0 通过**,下一步做 P2.1(真正过滤 getInstalledPackages)。
- 第 2 轮**bootloop / system_server 乱重启 / 明显卡顿** → 注入 system_server 当前方式代价太大或不可行。设备会自动恢复(标志一次性,已消费),把现象告诉我,我来定是改成更省的只拦特定事务、还是另寻路子。

## 出问题怎么发回
```bat
adb shell heraldctl log > brokerlog.txt 2>&1
adb shell heraldctl diag > diag.txt 2>&1
adb shell "ps -A" > ps.txt 2>&1
adb shell dmesg > dmesg.txt 2>&1
adb logcat -d > logcat.txt 2>&1
```
说明:卡在哪一步、现象、第 1 轮是否正常。
