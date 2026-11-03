use std::{
    string::{String, ToString},
    sync::OnceLock,
};

static ANDROID_MAJOR_VERSION: OnceLock<Option<i32>> = OnceLock::new();

pub fn android_major_version() -> Option<i32> {
    *ANDROID_MAJOR_VERSION.get_or_init(|| android_major_version_with(read_string_property))
}

pub fn android_major_version_with(read_property: impl Fn(&str) -> Option<String>) -> Option<i32> {
    read_property("ro.build.version.release_or_codename")
        .or_else(|| read_property("ro.build.version.release"))
        .and_then(|value| value.parse::<i32>().ok())
        .or_else(|| {
            read_property("ro.build.version.sdk")
                .and_then(|sdk| sdk.parse::<i32>().ok())
                .and_then(|sdk| match sdk {
                    31 | 32 => Some(12),
                    33 => Some(13),
                    34 => Some(14),
                    35 => Some(15),
                    36 => Some(16),
                    value if value >= 37 => Some(17),
                    // 旧平台(SDK < 31)或识别不了的版本:返回"未知"而不是 panic——
                    // 这段代码会被注入进 keystore2,panic 在 panic=abort 下会把整个 keystore 一起带崩。
                    _ => None,
                })
        })
}

fn read_string_property(name: &str) -> Option<String> {
    rsproperties::get::<String>(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
