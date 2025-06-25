use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ActiveLimitStatus {
    pub active_limit: u64,
    pub current_active_downloads: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MonthlyLimitStatus {
    pub monthly_limit: u64,
    pub current_downloads: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CooldownLimitStatus {
    pub cooldown_until: DateTime<FixedOffset>,
    pub current_time: DateTime<FixedOffset>,
}
