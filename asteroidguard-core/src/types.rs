#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub expected_self_hash_sha256: Option<[u8; 32]>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            expected_self_hash_sha256: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RiskReport {
    pub is_rooted: bool,
    pub is_debugging: bool,
    pub is_emulator: bool,
    pub hooking_detected: bool,
    pub integrity_ok: bool,
    pub risk_level: RiskLevel,
}

impl Default for RiskReport {
    fn default() -> Self {
        Self {
            is_rooted: false,
            is_debugging: false,
            is_emulator: false,
            hooking_detected: false,
            integrity_ok: true,
            risk_level: RiskLevel::Low,
        }
    }
}
