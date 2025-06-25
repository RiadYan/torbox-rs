use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebDownloadCreationResponse {
    pub hash: String,
    pub webdownload_id: String,
    pub auth_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebDownloadFile {
    pub id: u64,
    pub md5: String,
    pub s3_path: String,
    pub name: String,
    pub size: u64,
    pub mimetype: String,
    pub short_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebDownloadStatus {
    pub id: u64,
    pub hash: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub size: u64,
    pub active: bool,
    pub auth_id: String,
    pub download_state: String,
    pub progress: f64,
    pub download_speed: u64,
    pub upload_speed: u64,
    pub name: String,
    pub eta: u64,
    pub server: u64,
    pub torrent_file: bool,
    pub expires_at: DateTime<FixedOffset>,
    pub download_present: bool,
    pub download_finished: bool,
    pub error: String,
    pub files: Vec<WebDownloadFile>,
    pub inactive_check: u64,
    pub availability: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct HosterInfo {
    pub name: String,

    #[serde(default)]
    pub domains: Vec<String>,

    #[serde(default, rename = "domais")]
    pub domais: Option<Vec<String>>,

    pub url: String,
    pub icon: String,
    pub status: bool,

    #[serde(rename = "type")]
    pub kind: String,

    pub note: Option<String>,
    pub daily_link_limit: u64,
    pub daily_link_used: u64,
    pub daily_bandwidth_limit: u64,
    pub daily_bandwidth_used: u64,

    #[serde(default)]
    pub limit: Option<u64>,
}
