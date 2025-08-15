use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorboxGeneralStats {
    #[serde(default)]
    pub total_downloads: Option<u64>,
    #[serde(default)]
    pub total_users: Option<u64>,
    #[serde(default)]
    pub total_bytes_downloaded: Option<u64>,
    #[serde(default)]
    pub total_bytes_uploaded: Option<u64>,
    #[serde(default)]
    pub active_torrents: Option<u64>,
    #[serde(default)]
    pub active_usenet_downloads: Option<u64>,
    #[serde(default)]
    pub active_web_downloads: Option<u64>,
    #[serde(default)]
    pub total_usenet_downloads: Option<u64>,
    #[serde(default)]
    pub total_torrent_downloads: Option<u64>,
    #[serde(default)]
    pub total_web_downloads: Option<u64>,
    #[serde(default)]
    pub total_servers: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChangelogJsonVersion {
    pub name: String,
    pub html: String,
    pub markdown: String,
    pub link: String,
    pub created_at: DateTime<FixedOffset>,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SpeedtestFile {
    pub region: String,
    pub name: String,
    pub domain: String,
    pub path: String,
    pub url: String,
    pub closest: bool,
    pub coordinates: Coordinates,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Coordinates {
    pub lat: f32,
    pub lng: f32,
}
