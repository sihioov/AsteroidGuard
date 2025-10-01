mod types;
mod env;

pub use types::{RiskLevel, RiskReport, SecurityConfig};
use env::{DefaultEnv, Env};

pub fn check_all(_cfg: &SecurityConfig) -> RiskReport {

    RiskReport::default()
}

pub fn is_rooted() -> bool {
    is_rooted_with(&DefaultEnv {})
}

pub fn is_debugging() -> bool {
    is_debugging_with(&DefaultEnv {})
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

fn is_rooted_with<E: Env>(env: &E) -> bool {
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
    if su_paths.iter().any(|p| env.file_exists(p)) {
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
    if root_artifacts.iter().any(|p| env.file_exists(p)) {
        return true;
    }

    // build tags 확인
    if let Some(tags) = env.getprop("ro.build.tags") {
        if tags.contains("test-keys") {
            return true;
        }
    }

    // su 실행 가능 여부 확인(간이 which)
    if env.which("su") {
        return true;
    }

    false
}

fn is_debugging_with<E: Env>(env: &E) -> bool {
    // 안드로이드 시스템 속성 확인
    if let Some(v) = env.getprop("ro.debuggable") {
        if v == "1" {
            return true;
        }
    }
    if let Some(v) = env.getprop("ro.secure") {
        if v == "0" {
            return true;
        }
    }

    // 리눅스/안드로이드에서 TracerPid 확인 (디버거가 붙어 있으면 > 0)
    if let Some(tracer_pid) = tracer_pid_from_proc_status() {
        if tracer_pid > 0 {
            return true;
        }
    }

    false
}

#[cfg(unix)]
fn tracer_pid_from_proc_status() -> Option<u32> {
    use std::fs;
    let content = fs::read_to_string("/proc/self/status").ok()?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("TracerPid:") {
            let pid_str = rest.trim();
            if let Ok(pid) = pid_str.parse::<u32>() {
                return Some(pid);
            }
        }
    }
    None
}

#[cfg(not(unix))]
fn tracer_pid_from_proc_status() -> Option<u32> { None }

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    struct FakeEnv {
        files: HashSet<String>,
        props: HashMap<String, String>,
        bins: HashSet<String>,
    }

    impl Env for FakeEnv {
        fn file_exists(&self, path: &str) -> bool {
            self.files.contains(path)
        }
        fn getprop(&self, key: &str) -> Option<String> {
            self.props.get(key).cloned()
        }
        fn which(&self, bin: &str) -> bool {
            self.bins.contains(bin)
        }
    }

    #[test]
    fn rooted_by_su_file() {
        let mut f = FakeEnv { files: Default::default(), props: Default::default(), bins: Default::default() };
        f.files.insert("/system/xbin/su".into());
        assert!(is_rooted_with(&f));
    }

    #[test]
    fn rooted_by_test_keys() {
        let mut f = FakeEnv { files: Default::default(), props: Default::default(), bins: Default::default() };
        f.props.insert("ro.build.tags".into(), "release-keys test-keys".into());
        assert!(is_rooted_with(&f));
    }

    #[test]
    fn rooted_by_which_su() {
        let mut f = FakeEnv { files: Default::default(), props: Default::default(), bins: Default::default() };
        f.bins.insert("su".into());
        assert!(is_rooted_with(&f));
    }

    #[test]
    fn not_rooted_default() {
        let f = FakeEnv { files: Default::default(), props: Default::default(), bins: Default::default() };
        assert!(!is_rooted_with(&f));
    }

    #[test]
    fn debugging_by_ro_debuggable() {
        let mut f = FakeEnv { files: Default::default(), props: Default::default(), bins: Default::default() };
        f.props.insert("ro.debuggable".into(), "1".into());
        assert!(is_debugging_with(&f));
    }

    #[test]
    fn debugging_by_ro_secure_zero() {
        let mut f = FakeEnv { files: Default::default(), props: Default::default(), bins: Default::default() };
        f.props.insert("ro.secure".into(), "0".into());
        assert!(is_debugging_with(&f));
    }
}
