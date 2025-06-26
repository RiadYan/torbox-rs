use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserProfile {
    pub id: u64,
    pub auth_id: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub plan: u8,
    pub total_downloaded: u64,
    pub customer: String,
    pub is_subscribed: bool,
    pub premium_expires_at: DateTime<FixedOffset>,
    pub cooldown_until: DateTime<FixedOffset>,
    pub email: String,
    pub user_referral: String,
    pub base_email: String,
    pub server: Option<u64>,
    pub total_bytes_downloaded: u64,
    pub total_bytes_uploaded: u64,
    pub torrents_downloaded: u64,
    pub web_downloads_downloaded: u64,
    pub usenet_downloads_downloaded: u64,
    pub additional_concurrent_slots: u64,
    pub long_term_seeding: bool,
    pub long_term_storage: bool,
    pub is_vendor: bool,
    pub vendor_id: Option<String>,
    pub purchases_referred: u64,
    #[serde(default)]
    pub settings: Option<HashMap<String, Value>>,
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
