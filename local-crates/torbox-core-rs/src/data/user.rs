use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserProfile {
    pub id: u64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub email: String,
    pub plan: u8,
    pub total_downloaded: u64,
    pub customer: String,
    pub server: u64,
    pub is_subscribed: bool,
    pub premium_expires_at: DateTime<FixedOffset>,
    pub cooldown_until: DateTime<FixedOffset>,
    pub auth_id: String,
    pub user_referral: String,
    pub base_email: String,
    pub settings: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Job {
    pub id: u64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub auth_id: String,
    pub hash: String,
    #[serde(rename = "type")]
    pub job_type: String,
    pub integration: String,
    #[serde(default)]
    pub file_id: Option<u64>,
    pub zip: bool,
    pub progress: f64,
    pub detail: String,
    pub download_url: Option<String>,
    pub status: String,
}
