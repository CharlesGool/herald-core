# 注入器能力化设计(草案)

> 状态:**已评审通过,实施中**。**P1a 完整**(install 阶段接缝 + binder 热路径分发 → `Interceptor` trait,keystore 作第一个实现)已完成并在 ubt-dev 编译通过、行为保持;operation-publication / pending 帧 / 合成节点 retire 等机器按计划留在核心。**待真机三绿验证**(test-v1.3.3)。P1b(SOTER 作第二实现)、P2(应用列表隐藏)按需推进。

把 native 注入层从"单体的 keybox 认证引擎"重构成「**能力基座 + 扩展**」:基座只提供"坐进系统服务、拦改其 binder 事务"的无偏好能力,keybox/认证作为**第一个扩展**接进来。与 broker(shell)层已有的"核心 + 插件接口"同构。

## 背景

现状分两层:

| 层 | 现状 | 是否已分离 |
|---|---|---|
| patch / 部署 | 核心 `apply.sh` 只放 `bin/resetprop`;`keymint`/`inject` 二进制、keybox 种子由 **attestation-fix** 插件部署并 `HERALDP_START` 启动 | **已分**:只打核心、不打 attestation-fix,设备上无 keybox 能力 |
| 二进制 | `engine/` 编出的 `keymint` 是单体,keybox/认证逻辑编死在 `keymaster/`、`security_level`、`keybox.rs` 里;`inject` 的 `identify`/`hook/rewrite` 写死 keystore2 | **未分**:本设计要解决的层 |

`inject` 内部其实已是"通用下层 + keystore 专用上层"的雏形:

- 通用下层(可复用):`inject`(ptrace 进目标)、`hook/intercept/{fd,ioctl}`(拦 binder fd)、`parcel/`(parcel 读写)、分发。
- keystore 专用上层:`identify`(写死 `android.system.keystore2.*` 接口名与事务码,按 `android_major_version` 分版本)、`hook/rewrite/*`(KeyMint parcel 与 operation 路由)。

## 设计原则

1. **基座无能力偏好**:默认是一个**透明的中间人**——坐在目标服务的 binder 路径上,默认透传,对每个事务提供 hook 点,本身不带任何认证/伪装策略。
2. **扩展接入**:具体行为(keybox 认证、按 App 管控…)由扩展按统一契约接入。
3. **per-AIDL 不可消除**:parcel 布局随服务和版本而异,基座只能给读写原语和分发框架,**给不了"零代码接任意服务"**。每个扩展必须自带其目标的 parcel 知识。

## 架构

### 能力基座(capability core)

承担与目标无关的全部机制:进程注入、binder fd 拦截、parcel 读写原语、事务分发、透传、生命周期。对外暴露:

- `ParcelReader` / `ParcelWriter`:按位置读写 binder parcel 字段的原语;
- 分发器:把拦到的事务按"目标 + 接口 + 事务码"路由给已注册的扩展;未匹配则透传。

### 扩展契约(编译期 trait + 注册表)

```text
trait Interceptor {
    fn targets(&self) -> &[Target];      // 注入哪个进程、盯哪些接口名 + 事务码(按安卓版本给表)
    fn on_request(&self, tx: &Transaction, p: &mut Parcel) -> Action;   // 放行 / 改写 / 合成应答 / 拦截
    fn on_reply(&self,   tx: &Transaction, p: &mut Parcel) -> Action;   // 放行 / 改写
    fn init(&mut self, ctx: &Ctx) -> Result<()>;
    fn teardown(&mut self);
}
```

- 接入方只写自己那一摊(target 表 + parcel 改写);ptrace/binder/parcel 由基座统一提供。
- **编译期注册**:每个扩展一个模块实现 `Interceptor`,登记进注册表。加扩展要重编二进制——因为是原生 binder 手术,躲不开编译(不像 broker 的 shell 插件能热插)。
- 可选的**声明式 config** 覆盖简单场景("接口 X 事务 Y 把字段 Z 换成 …"),不重编;复杂的(如 attestation)仍回到 Rust。

### keybox-attestation 作首个扩展

现有 keystore2 认证逻辑整体收敛成**扩展 #1**:`targets()` 返回现在 `identify` 里那套 keystore2 接口/事务码;`on_request`/`on_reply` 即现在的 `hook/rewrite/*`;keybox 加载、RKP 仍在扩展内部。**行为须与现状逐字节一致**(三绿靠它)。

### 版本处理

事务码随安卓版本变(`identify` 已按 `android_major_version` 分版本)。`Target` 携带 per-version 事务映射,由各扩展维护。

## 边界与非目标

- **本轮只做 `inject` 侧**:基座 = 拦截能力 + 扩展契约;keystore 认证迁成扩展 #1。
- **`keymint` 内部暂不动**:把 keybox/attestation 从 KeyMint TA 里剥成可插拔"后端"是另一个大重写;**当前只有 keybox 一个后端,为单插件造插件系统是过度设计**,推迟到真有第二个后端再做。
- 不做:零代码接任意服务;动态 `.so`/脚本插件(Android 上 exec/SELinux/ABI 太脆)。

## 迁移路径

1. 抽出基座(把 `inject`/`hook/intercept`/`parcel` 与 `identify`/`hook/rewrite` 切开),定义 `Interceptor` trait + 注册表;
2. 把 keystore 认证改造成实现该 trait 的**扩展 #1**,其余逻辑原样搬进去;
3. 证明认证行为不变(离线对拍 + **真机三绿**);
4. 之后按同契约加**第二个扩展:应用列表隐藏**(拦 system_server 的 `IPackageManager`),验证契约能否从 keystore2 迈到 system_server。

## 风险与约束

- **设备关键**:改的是三绿认证路径,任何重构都要保行为不变,且**离不开真机验证**。
- **SELinux**:注入靠 herald 域 permissive 兜底;注入别的进程要其可达,加固过的更难。
- **脆**:强依赖具体 AIDL + 版本。
- **许可**:`engine/` 属 Oh My Keymint 的 fork,**GLA v4 + Oh My Keymint License —— 仅非商业、改须附修改声明、不得用原名**。本重构属"修改"。

## 评审决定

| 问题 | 决定 | 原因 |
|---|---|---|
| parcel 原语粒度:字段级 vs 整条 parcel | **字段级 + 游标**(按偏移按类型读写、定位替换具体字段),不做整条 parcel 的完整 AIDL 反序列化 | 现有 rewrite 本就是外科式改特定字段;整条重建既脆又随版本漂,收益低 |
| 声明式 config | **暂不做,全走 Rust** | 唯一现存扩展(keybox 认证)远超配置能表达;没有具体的简单用例就造 DSL 属投机。等真出现简单扩展再回头看 |
| 第二个扩展选谁验证契约 | **应用列表隐藏**(拦 `IPackageManager` 的包查询、过滤返回项) | 是干净的"改应答"用例、非认证、不破坏性,正好填你现在走 services.jar 没做成的需求。**注意**:它在 system_server 里,P2 真正要验证的就是"框架能否从 keystore2 迈到 system_server 注入"——这也是它的风险点 |

## 待决问题

- 以上决定若有异议请指出;否则据此把本草案结论并入 [DESIGN.md](DESIGN.md) 后开工。
