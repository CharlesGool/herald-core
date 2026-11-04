//! keystore 认证扩展接入核心的 [`Interceptor`] 实现。
//!
//! P1a 只实现安装阶段的接缝:各方法只**转调现有函数**,逻辑与改前逐字节一致
//! (对照 `install.rs` 改前的 `if !soter { … }` 四处调用)。

use crate::intercept_api::Interceptor;

pub(crate) struct KeystoreInterceptor;

static KEYSTORE_INTERCEPTOR: KeystoreInterceptor = KeystoreInterceptor;

/// 供入口注册用:keystore 扩展的 `'static` 引用。
pub(crate) fn keystore_interceptor() -> &'static dyn Interceptor {
    &KEYSTORE_INTERCEPTOR
}

impl Interceptor for KeystoreInterceptor {
    fn before_hooks(&self) -> anyhow::Result<()> {
        // 注入线程可能挂在 libbinder 里,ServiceManager 调用需要新的 IPCThreadState。
        crate::ipc::ensure_process_state();
        super::binder::start_registered_service_target_worker()?;
        Ok(())
    }

    fn after_hooks(&self) -> anyhow::Result<()> {
        super::intercept::start_operation_publication_worker().map_err(|e| anyhow::anyhow!(e))?;
        super::rewrite::start_mirror_recovery_worker().map_err(|e| anyhow::anyhow!(e))?;
        Ok(())
    }
}
