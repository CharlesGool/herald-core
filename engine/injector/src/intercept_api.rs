//! 注入扩展的接入点:能力基座 ↔ 扩展的接缝。
//!
//! 核心(ptrace 注入、binder fd/ioctl 拦截、parcel 搬运)本身对"注入什么"没有偏好;
//! 具体目标(当前仅 keystore 认证)作为实现 [`Interceptor`] 的扩展接入。详见
//! `docs/injector-capability-design.md`。
//!
//! P1a 只把**一次性安装阶段**的接缝抽出来(`before_hooks`/`after_hooks`);binder 热路径
//! (请求/应答分发)仍是核心直接调 keystore,后续增量再迁。未注册扩展时核心按
//! "无能力偏好"运行(安装阶段不做任何扩展副作用)。

use std::sync::OnceLock;

use crate::hook::binder::binder_transaction_data;
use crate::hook::BinderStateKey;

/// 一个注入扩展。核心在固定的生命周期点回调它。
pub trait Interceptor: Sync {
    /// PLT 挂钩**安装之前**的一次性准备(进程态、注册服务身份等)。
    fn before_hooks(&self) -> anyhow::Result<()> {
        Ok(())
    }
    /// PLT 挂钩**提交之后**拉起扩展自己的后台 worker。
    fn after_hooks(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// 入站 binder 事务分发。返回 `true`=已改写(核心需装 shadow),`false`=透传。
    /// 未注册扩展时核心按默认透传(`false`)。
    ///
    /// # Safety
    /// `tr` 必须指向当前事务的有效 `binder_transaction_data`。
    unsafe fn on_incoming_transaction(
        &self,
        connection: BinderStateKey,
        tr: &mut binder_transaction_data,
        caller_sid: Option<String>,
        command_name: &str,
    ) -> bool {
        let _ = (connection, tr, caller_sid, command_name);
        false
    }

    /// 出站 `BC_REPLY` 分发。返回改写的帧 id,或 `None`。
    ///
    /// # Safety
    /// 同 [`Interceptor::on_incoming_transaction`]。
    unsafe fn on_outgoing_reply(
        &self,
        connection: BinderStateKey,
        tr: &mut binder_transaction_data,
    ) -> Option<u64> {
        let _ = (connection, tr);
        None
    }
}

static INTERCEPTOR: OnceLock<&'static dyn Interceptor> = OnceLock::new();

/// 注册本进程的拦截扩展(P1a 固定为 keystore,只注册一次)。
pub fn register(interceptor: &'static dyn Interceptor) {
    let _ = INTERCEPTOR.set(interceptor);
}

/// 取当前已注册的扩展;未注册则 `None`(核心不做扩展副作用)。
pub fn get() -> Option<&'static dyn Interceptor> {
    INTERCEPTOR.get().copied()
}
