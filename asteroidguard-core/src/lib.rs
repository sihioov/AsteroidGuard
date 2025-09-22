mod types;
mod detectors;
mod types;

pub use types::{RiskLevel, RiskReport};

pub fn is_rooted() -> bool {
    false
}