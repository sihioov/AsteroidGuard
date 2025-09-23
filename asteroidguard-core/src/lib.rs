mod types;

pub use types::{RiskLevel, RiskReport, SecurityConfig};

pub fn check_all(cfg: &SecurityConfig) -> RiskReport {

    RiskReport::default()
}

pub fn is_rooted() -> bool {
    false
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

pub fn verify_integrity(cfg: &SecurityConfig) -> bool {
    true
}

