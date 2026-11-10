use std::ffi::c_void;

use anyhow::{bail, Context};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use kmr_common::consts::{KEYSTORE_GID, KEYSTORE_UID};
use log::{error, info, warn, LevelFilter};
use nix::unistd::Pid;

use crate::android::hardware::security::keymint::SecurityLevel::SecurityLevel;

mod auto_targets;
pub mod config;
pub mod filter;
pub mod forward;
pub mod hook;
pub mod identify;
pub mod inject;
pub mod intercept_api;
pub mod ipc;
pub mod legacy;
pub mod logging;
pub mod parcel;
pub mod sys;
pub mod tracker;
pub mod utils;

include!(concat!(env!("OUT_DIR"), "/aidl.rs"));

fn log_runtime_identity(role: &str) {
    let uid = unsafe { libc::getuid() };
    let euid = unsafe { libc::geteuid() };
    let gid = unsafe { libc::getgid() };
    let egid = unsafe { libc::getegid() };
    info!(
        "runtime identity role={} uid={} euid={} gid={} egid={}",
        role, uid, euid, gid, egid
    );
}

const WEBUI_GET_TEE_STATUS: &str = "--webui-get-tee-status";

fn parse_webui_tee_status_args(args: &[String]) -> Option<Result<(), String>> {
    let command = args.first()?;
    if command != WEBUI_GET_TEE_STATUS {
        return None;
    }
    if args.len() != 1 {
        return Some(Err(format!(
            "{WEBUI_GET_TEE_STATUS} does not accept arguments"
        )));
    }
    Some(Ok(()))
}

fn enter_keystore_identity() -> anyhow::Result<()> {
    let effective_uid = unsafe { libc::geteuid() };
    let effective_gid = unsafe { libc::getegid() };
    if effective_uid == KEYSTORE_UID && effective_gid == KEYSTORE_GID {
        return Ok(());
    }
    if effective_uid != 0 {
        bail!(
            "TEE status probe requires root or keystore uid/gid; current euid={effective_uid} egid={effective_gid}"
        );
    }

    if unsafe { libc::setgid(KEYSTORE_GID) } != 0 {
        return Err(std::io::Error::last_os_error())
            .context("failed to enter keystore gid for TEE status probe");
    }
    if unsafe { libc::setuid(KEYSTORE_UID) } != 0 {
        return Err(std::io::Error::last_os_error())
            .context("failed to enter keystore uid for TEE status probe");
    }
    if unsafe { libc::geteuid() } != KEYSTORE_UID || unsafe { libc::getegid() } != KEYSTORE_GID {
        bail!("TEE status probe did not enter the keystore uid/gid");
    }
    Ok(())
}

fn probe_tee_status() -> anyhow::Result<()> {
    enter_keystore_identity()?;
    ipc::ensure_process_state();
    ipc::with_herald_once(|herald| {
        let _ = herald.r#getSecurityLevel(SecurityLevel::TRUSTED_ENVIRONMENT)?;
        Ok(())
    })
    .context("failed to obtain the HERALD TEE security level")
}

fn handle_webui_tee_status_command() -> Option<Result<&'static str, String>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match parse_webui_tee_status_args(&args)? {
        Ok(()) => Some(
            probe_tee_status()
                .map(|()| "normal")
                .map_err(|error| format!("{error:#}")),
        ),
        Err(error) => Some(Err(error)),
    }
}

fn handle_webui_config_command() -> Option<Result<String, String>> {
    let mut args = std::env::args();
    let _program = args.next();
    let command = args.next()?;

    match command.as_str() {
        "--webui-get-auto-targets" => {
            if args.next().is_some() {
                return Some(Err(
                    "--webui-get-auto-targets does not accept arguments".into()
                ));
            }
            Some(
                auto_targets::status()
                    .and_then(|state| Ok(serde_json::to_string(&state)?))
                    .map_err(|error| format!("{error:#}")),
            )
        }
        "--webui-set-auto-targets" => {
            let enabled = match args.next().as_deref() {
                Some("0") => false,
                Some("1") => true,
                _ => return Some(Err("--webui-set-auto-targets requires 0 or 1".into())),
            };
            if args.next().is_some() {
                return Some(Err("--webui-set-auto-targets accepts one argument".into()));
            }
            Some(
                auto_targets::set_enabled(enabled)
                    .and_then(|state| Ok(serde_json::to_string(&state)?))
                    .map_err(|error| format!("{error:#}")),
            )
        }
        "--webui-update-scoop" => {
            let Some(encoded) = args.next() else {
                return Some(Err(
                    "--webui-update-scoop requires a base64 JSON delta".into()
                ));
            };
            if args.next().is_some() || encoded.len() > 4 * 1024 * 1024 {
                return Some(Err("invalid scoop delta arguments".into()));
            }
            Some(
                BASE64_STANDARD
                    .decode(encoded)
                    .map_err(|error| error.to_string())
                    .and_then(|payload| {
                        serde_json::from_slice::<config::ScoopDelta>(&payload)
                            .map_err(|error| error.to_string())
                    })
                    .and_then(config::update_scoop_for_webui)
                    .and_then(|targets| {
                        serde_json::to_string(&targets).map_err(|error| error.to_string())
                    }),
            )
        }
        "--webui-get-scoop" => {
            if args.next().is_some() {
                return Some(Err(
                    "--webui-get-scoop does not accept arguments".to_string()
                ));
            }
            Some(
                config::read_scoop_for_webui().and_then(|packages| {
                    serde_json::to_string(&packages).map_err(|e| e.to_string())
                }),
            )
        }
        "--webui-set-scoop" => {
            let Some(encoded) = args.next() else {
                return Some(Err(
                    "--webui-set-scoop requires a base64-encoded JSON array".to_string(),
                ));
            };
            if args.next().is_some() {
                return Some(Err("--webui-set-scoop accepts one argument".to_string()));
            }
            let result = BASE64_STANDARD
                .decode(encoded)
                .map_err(|error| format!("invalid scoop payload encoding: {error}"))
                .and_then(|payload| {
                    serde_json::from_slice::<Vec<String>>(&payload)
                        .map_err(|error| format!("invalid scoop payload: {error}"))
                })
                .and_then(config::replace_scoop_for_webui)
                .map(|()| "ok".to_string());
            Some(result)
        }
        _ => None,
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--auto-targets-watch") {
        if std::env::args().count() != 2 {
            eprintln!("--auto-targets-watch does not accept arguments");
            std::process::exit(2);
        }
        logging::init_logger_fallback(LevelFilter::Warn);
        if let Err(error) = auto_targets::run_watcher() {
            eprintln!("automatic app selection watcher failed: {error:#}");
            std::process::exit(2);
        }
        return;
    }
    if let Some(result) = handle_webui_tee_status_command() {
        match result {
            Ok(output) => println!("{output}"),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(2);
            }
        }
        return;
    }

    if let Some(result) = handle_webui_config_command() {
        match result {
            Ok(output) => println!("{output}"),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(2);
            }
        }
        return;
    }

    logging::init_logger_fallback(LevelFilter::Debug);
    let config = config::get();
    if config::parse_level_filter(&config.main.log_level).is_none() {
        warn!(
            "injector logging unknown log level '{}', keeping debug fallback",
            config.main.log_level
        );
    }
    log::set_max_level(config.main.log_level_filter());
    log_runtime_identity("Launcher");
    match utils::current_exe_identity() {
        Ok(identity) => {
            info!(
                "injector binary build_id={} build_target={} git_sha={} runtime_arch={} exe={} sha256={} elf={}",
                utils::build_id(),
                utils::build_target(),
                utils::build_git_sha(),
                std::env::consts::ARCH,
                identity.path.display(),
                identity.sha256,
                identity.elf,
            );
        }
        Err(error) => {
            error!("failed to describe current injector binary: {:#}", error);
        }
    }

    // 目标进程选择:无参默认 keystore2(认证注入,行为不变);
    // `--target system_server` 注入 system_server(P2 应用列表隐藏)。
    let target_name = match std::env::args().nth(1).as_deref() {
        Some("--target") => match std::env::args().nth(2).as_deref() {
            Some("keystore2") => "keystore2",
            Some("system_server") => "system_server",
            other => {
                eprintln!("--target expects keystore2 or system_server, got {other:?}");
                std::process::exit(2);
            }
        },
        _ => "keystore2",
    };
    let (pid, target_path) = match utils::find_process_by_name(target_name) {
        Ok(target) => target,
        Err(error) => {
            warn!("event=target_not_ready process={target_name} error={error:#}; daemon will retry");
            std::process::exit(1);
        }
    };
    match utils::executable_identity(&target_path) {
        Ok(identity) => info!(
            "{target_name} target pid={} exe={} sha256={} elf={}",
            pid,
            identity.path.display(),
            identity.sha256,
            identity.elf,
        ),
        Err(error) => error!(
            "failed to describe {target_name} executable {}: {:#}",
            target_path.display(),
            error
        ),
    }
    let pid = Pid::from_raw(pid);
    match inject::inject_library(pid) {
        Ok(()) => info!("injection completed"),
        Err(e) => {
            error!("injection failed: {:#}", e);
            std::process::exit(1);
        }
    }
}

#[no_mangle]
#[allow(unused)]
pub extern "C" fn entry(handle: *const c_void) -> bool {
    // This runs inside the target process, so we must initialize logging again
    // for that process. On Android this enables both logcat and stdout logging.
    logging::init_logger_fallback(LevelFilter::Debug);
    // 载荷随 keystore2 常驻,但 herald 重启会重新注入;常驻的配置监听线程会在旧载荷映射失效后
    // 崩掉 keystore2(见 config::disable_watcher)。所以在第一次读 config 之前就关掉监听,
    // 重新注入时新载荷自然会重读配置。
    config::disable_watcher();
    let config = config::get();
    if config::parse_level_filter(&config.main.log_level).is_none() {
        warn!(
            "injector logging unknown log level '{}', keeping debug fallback",
            config.main.log_level
        );
    }
    log::set_max_level(config.main.log_level_filter());
    log_runtime_identity("Payload");
    log::info!(
        "Injected library entry called! Handle: {:?}, build_id={}, build_target={}, runtime_arch={}, current_exe={}",
        handle,
        utils::build_id(),
        utils::build_target(),
        std::env::consts::ARCH,
        utils::current_exe_path()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| "<unknown>".to_string()),
    );
    // 按被注入进程选扩展:载荷在目标进程里重新 dlopen,拿不到 launcher 的参数,
    // 只能从自身进程名判定。system_server = 应用列表隐藏(P2),其余 = keystore 认证。
    let self_name = utils::process_name(std::process::id() as i32).unwrap_or_default();
    let interceptor = if self_name == "system_server" {
        // 应用列表隐藏不连 keymint 后端,不建 HERALD RPC 会话(建了反而可能失败/阻塞)。
        hook::pkgmgr_interceptor()
    } else {
        // keystore 认证扩展要经 HERALD RPC 会话连 keymint 后端。
        if let Err(error) = ipc::install_direct_rpc_session() {
            error!("failed to initialize HERALD RPC session: {error:#}");
            return false;
        }
        hook::keystore_interceptor()
    };
    // core 的 install 阶段会回调注册的扩展(见 intercept_api、docs/injector-capability-design.md)。
    intercept_api::register(interceptor);
    if let Err(error) = hook::init_hook() {
        error!("failed to initialize binder ioctl hook: {error:#}");
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn tee_status_command_is_dispatched_without_arguments() {
        assert_eq!(
            parse_webui_tee_status_args(&args(&[WEBUI_GET_TEE_STATUS])),
            Some(Ok(()))
        );
    }

    #[test]
    fn tee_status_command_rejects_extra_arguments() {
        assert_eq!(
            parse_webui_tee_status_args(&args(&[WEBUI_GET_TEE_STATUS, "extra"])),
            Some(Err(format!(
                "{WEBUI_GET_TEE_STATUS} does not accept arguments"
            )))
        );
    }

    #[test]
    fn tee_status_parser_leaves_other_commands_for_existing_dispatch() {
        assert_eq!(
            parse_webui_tee_status_args(&args(&["--webui-get-scoop"])),
            None
        );
        assert_eq!(parse_webui_tee_status_args(&[]), None);
    }
}
