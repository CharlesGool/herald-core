# herald 安全加固 + 轮询优化 真机测试

这次刷的包改了两处,目标都是**不改变正常使用**,所以重点是"别回归",外加几项能直接观察的新行为。
- broker(herald-start.sh / heraldctl):自适应轮询(更快)、日志移到 root 私有目录(`/data/adb/herald`)、
  写回包/写配对 token 加 O_EXCL、Settings 键名校验、heraldctl log 改成经 broker 读。
  工作目录 `.work` 仍留在命令行通道(`/data/local/tmp/herald/.work`,root 0700)——回包要带 shell 标签,heraldctl 才读得到(V1.2 曾误移到 /data/adb 导致回包全空,V1.3 已移回)。
- engine(keymint / inject):未知 Android 版本不再 panic、keymint.dat 原子写、
  keybox 解析失败不再覆盖磁盘文件。

环境:Windows cmd,adb 已连上(`adb shell echo ok` 有输出)。命令不要用管道;要存日志就用 `> 文件名`。

---

## 1. 后端自检(最重要,一条就能看大面)

```
adb shell heraldctl selftest
```
期望:
- `uid=0`、`ctx=u:r:herald:s0`、`selinux=Enforcing`
- `plugins=` 列出 `00-attestation.sh navbar.sh tango.sh`
- `settings_put=ok`
- `keymint=present`
- 末尾各插件自检没有 FAIL/MISSING

任一项不对 → 存下来发回:`adb shell heraldctl selftest > selftest.txt 2>&1`

## 2. 日志移到 root 私有 + heraldctl log 经 broker 读(本次改动)

```
adb shell heraldctl spoof get
adb shell heraldctl log
```
期望:`heraldctl log` 能列出刚才这条命令的记录,**不是**"(还没有日志)"。

```
adb shell ls -la /data/local/tmp/herald
```
期望:能看到 `.work` 目录(root 所有,shell 进不去是正常的),以及可能有瞬时的 `req.*` / `resp.*`。
这里**不该有正在写的 `broker.log`**——当前日志在 `/data/adb/herald`(shell 读不到),靠上面的 `heraldctl log` 经 broker 读。
若看到一个旧的 `broker.log`(时间戳比刚才的操作早),是旧版本包的残留,无害,可 `adb shell rm /data/local/tmp/herald/broker.log` 清掉。

## 3. 轮询变快(手感,本次改动)

连着跑几条,应几乎瞬间返回(以前每条要等约 0.5–1.5 秒):
```
adb shell heraldctl pair
adb shell heraldctl spoof get
adb shell heraldctl navbar status
adb shell heraldctl tango status
```

## 4. 伪装 / 认证只读检查(覆盖 engine keybox、版本,以及三绿)

```
adb shell heraldctl spoof get
adb shell heraldctl spoof list
adb shell heraldctl keybox state
adb shell heraldctl diag
```
期望:
- `spoof get` 输出 JSON,末尾单独一行 `HERALD_GLOBAL_PIF_FLAG=on` 或 `off`
- `spoof list` 是 JSON 数组
- `keybox state` 正常(valid、未吊销)
- `diag` 里 keymint 日志末尾没有 fatal/崩溃

然后用你惯用的 Play Integrity 检测器确认 **三绿 / 认证维持原状**(和这次刷机前一致)。
> 这一节只读,不要在测试里 `spoof set` / `spoof off` 去改状态。

## 5. 各插件命令正常(回归)

```
adb shell heraldctl navbar status
adb shell heraldctl navbar hide
adb shell heraldctl navbar show
adb shell heraldctl tango status
```
期望:navbar hide 能隐藏导航栏、show 恢复;tango status 正常输出。
(如果这个包装了 herald-readonly-shell,再测:`adb shell heraldctl rexec on 5` → `adb shell heraldctl rexec cat /proc/version` → `adb shell heraldctl rexec off`。没装就跳过。)

## 6. 重启后持久(覆盖 keymint.dat 原子写,重点)

```
adb reboot
```
等开机完成,再:
```
adb shell heraldctl selftest
adb shell heraldctl keybox state
```
再用 Play Integrity 检测一次。
期望:**密钥没丢、仍三绿**。连续重启 2–3 次都应如此(这是在验证 keymint.dat 不会因重启损坏)。

## 7. 前端 App 配对(如果有前端 App)

```
adb shell heraldctl pair <App 包名>
```
期望:`ok paired ...`;随后 App 内能正常收发命令(覆盖写回包 O_EXCL 不影响正常流程)。

---

## 出问题怎么发回

把相关命令的完整输出各存一个文件发回(cmd 下用 `>`,别用管道):
```
adb shell heraldctl selftest > selftest.txt 2>&1
adb shell heraldctl diag > diag.txt 2>&1
adb shell heraldctl log > brokerlog.txt 2>&1
adb shell dmesg > dmesg.txt 2>&1
```
顺便说明:是哪一步、现象是什么、刷机前是否正常。
