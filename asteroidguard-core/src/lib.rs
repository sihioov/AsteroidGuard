mod types;

pub use types::{RiskLevel, RiskReport, SecurityConfig};

pub fn check_all(cfg: &SecurityConfig) -> RiskReport {

    RiskReport::default()
}

pub fn is_rooted() -> bool {
    is_rooted_impl()
}

pub fn is_debugging() -> bool {
    false
}

pub fn is_emulator() -> bool {
    false
}

pub fn detect_hooking() -> bool {
    false
}

pub fn verify_integrity(_cfg: &SecurityConfig) -> bool {
    true
}


#[cfg(target_os = "android")]
fn is_rooted_impl() -> bool {
    // su 바이너리 존재 여부 확인
    let su_paths = [
        "/system/bin/su",
        "/system/xbin/su",
        "/sbin/su",
        "/su/bin/su",
        "/vendor/bin/su",
        "/system/sd/xbin/su",
        "/system/bin/.ext/.su",
    ];
    if su_paths.iter().any(|p| std::path::Path::new(p).exists()) {
        return true;
    }

    // 루팅 관련 흔적 파일 확인
    let root_artifacts = [
        "/sbin/magisk",
        "/system/bin/magisk",
        "/data/adb/magisk",
        "/system/app/Superuser.apk",
        "/system/app/SuperSU.apk",
    ];
    if root_artifacts
        .iter()
        .any(|p| std::path::Path::new(p).exists())
    {
        return true;
    }

    // build tags 확인
    if let Ok(output) = std::process::Command::new("getprop")
        .arg("ro.build.tags")
        .output()
    {
        if let Ok(tags) = String::from_utf8(output.stdout) {
            if tags.contains("test-keys") {
                return true;
            }
        }
    }

    // su 실행 시도 
    if let Ok(output) = std::process::Command::new("which")
        .arg("su")
        .output()
    {
        if output.status.success() {
            return true;
        }
    }

    false
}

