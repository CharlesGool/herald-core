//! 应用列表隐藏扩展(P2)接入核心的 [`Interceptor`] 实现。
//!
//! P2.0 只做**透传**:验证把注入器坐进 system_server、挂上 binder ioctl hook 之后,
//! 系统能正常开机、system_server 不崩、三绿不回归。真正的 IPackageManager 应答过滤
//! (ParceledListSlice 解析 + 按包名删项)留到 P2.1,届时在这里实现
//! `on_incoming_transaction`(认 `android.content.pm.IPackageManager` + 事务码、压过滤帧)
//! 和 `on_outgoing_reply`(解析列表、删隐藏项、重建回包)。

use crate::intercept_api::Interceptor;

pub(crate) struct PkgManagerInterceptor;

static PKGMANAGER_INTERCEPTOR: PkgManagerInterceptor = PkgManagerInterceptor;

/// 供入口注册用:应用列表隐藏扩展的 `'static` 引用。
pub(crate) fn pkgmgr_interceptor() -> &'static dyn Interceptor {
    &PKGMANAGER_INTERCEPTOR
}

// P2.0:全部沿用 Interceptor 的默认实现——before_hooks/after_hooks 无副作用,
// on_incoming_transaction 返回 false(透传),on_outgoing_reply 返回 None(不改回包)。
// 即核心把 hook 挂进 system_server,但每个事务都原样放行。
impl Interceptor for PkgManagerInterceptor {}
