use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadCreationResponse {
    pub hash: String,
    pub webdownload_id: u32,
    pub auth_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadFile {
    pub id: u64,
    pub md5: Option<String>,
    pub s3_path: String,
    pub name: String,
    pub size: u64,
    pub zipped: bool,
    pub infected: bool,
    pub opensubtitles_hash: Option<String>,
    pub mimetype: String,
    pub short_name: String,
    pub absolute_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadStatus {
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
    pub original_url: String,
    pub name: String,
    pub eta: u64,
    pub server: u64,
    pub expires_at: Option<DateTime<FixedOffset>>,
    pub download_present: bool,
    pub download_finished: bool,
    pub error: Option<String>,
    pub cached: bool,
    pub cached_at: Option<DateTime<FixedOffset>>,
    pub download_id: Option<String>,
    pub files: Vec<WebdownloadFile>,
    pub alternative_hashes: Vec<String>,
    pub tags: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebDownloadCacheAvailability {
    pub name: String,
    pub size: u64,
    pub hash: String,

    pub files: Option<WebdownloadFile>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum WebdownloadHosterKind {
    Hoster,
    Stream,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadHosterList {
    pub id: u32,
    pub name: String,
    pub domains: Vec<String>,
    pub url: String,
    pub icon: String,
    pub status: bool,

    #[serde(rename = "type")]
    pub kind: WebdownloadHosterKind,

    pub note: Option<String>,
    pub nsfw: bool,

    pub daily_link_limit: u32,
    pub daily_link_used: u32,

    pub daily_bandwidth_limit: u64,
    pub daily_bandwidth_used: u64,
    pub per_link_size_limit: u64,

    pub regex: String,
}
