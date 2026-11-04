//! keystore 认证扩展接入核心的 [`Interceptor`] 实现。
//!
//! P1a 只实现安装阶段的接缝:各方法只**转调现有函数**,逻辑与改前逐字节一致
//! (对照 `install.rs` 改前的 `if !soter { … }` 四处调用)。

use super::binder::binder_transaction_data;
use super::BinderStateKey;
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

    unsafe fn on_incoming_transaction(
        &self,
        connection: BinderStateKey,
        tr: &mut binder_transaction_data,
        caller_sid: Option<String>,
        command_name: &str,
    ) -> bool {
        // 合成节点查表 + 压帧的判定属于 keystore 自己;命中则不改写、透传。
        let target = super::binder::LocalBinderTarget {
            ptr: tr.target.ptr,
            cookie: tr.cookie,
        };
        if super::rewrite::lookup_synthetic_target(target).is_some() {
            if (tr.flags & super::binder::TF_ONE_WAY) == 0 {
                super::rewrite::push_pending_frame(connection);
            }
            return false;
        }
        super::rewrite::handle_br_transaction(connection, tr, caller_sid, command_name)
    }

    unsafe fn on_outgoing_reply(
        &self,
        connection: BinderStateKey,
        tr: &mut binder_transaction_data,
    ) -> Option<u64> {
        super::rewrite::handle_bc_reply(connection, tr)
    }
}
