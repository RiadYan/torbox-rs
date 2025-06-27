use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentStatus {
    pub id: u64,
    pub hash: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub magnet: String,
    pub size: u64,
    pub active: bool,
    pub auth_id: String,
    pub download_state: String,
    pub seeds: u64,
    pub peers: u64,
    pub ratio: f64,
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
    pub files: Vec<TorrentFile>,
    pub inactive_check: u64,
    pub availability: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentMeta {
    pub name: String,
    pub hash: String,
    pub size: u64,
    pub trackers: Vec<String>,
    pub seeds: u64,
    pub peers: u64,
    pub files: Vec<TorrentFile>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentData {
    pub name: String,
    pub size: u64,
    pub hash: String,
    pub files: Vec<TorrentFile>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentFile {
    pub name: String,
    pub size: u64,
    pub hash: Option<String>,
}

pub type TorrentMap = HashMap<String, TorrentFile>;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TorrentDownloadState {
    Downloading,
    Uploading,
    Stalled,
    Paused,
    Completed,
    Cached,
    MetaDl,
    CheckingResumeData,
}
