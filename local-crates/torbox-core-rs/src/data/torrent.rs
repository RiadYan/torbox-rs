use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentStatus {
    pub id: u64,
    pub hash: String,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
    pub magnet: Option<String>,
    pub size: f64,
    pub active: bool,
    pub auth_id: String,
    pub download_state: TorrentDownloadState,
    pub seeds: u64,
    pub peers: u64,
    pub ratio: f64,
    pub progress: f64,
    pub download_speed: f64,
    pub upload_speed: f64,
    pub name: String,
    pub eta: f64,
    pub server: u64,
    pub torrent_file: bool,
    pub expires_at: Option<DateTime<FixedOffset>>,
    pub download_present: bool,
    pub download_finished: bool,
    pub files: Vec<TorrentFile>,
    pub inactive_check: Option<u64>,
    pub availability: f64,
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
    pub size: f64,
    pub hash: Option<String>,
}

pub type TorrentMap = HashMap<String, TorrentFile>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
/// All possible states of a torrent.
///
/// Here is where I got the comments for each variant : `https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1)#torrent-management`
pub enum TorrentDownloadState {
    // Error states
    /// Some error occurred, applies to paused torrents
    Error,
    /// Torrent data files is missing
    MissingFiles,

    // Uploading states
    /// Torrent is being seeded and data is being transferred
    Uploading,
    /// Torrent is being seeded, but no connection were made
    #[serde(rename = "uploading (no peers)", alias = "stalledUP")]
    UploadingNoPeers,
    /// Torrent is paused and has finished downloading
    #[serde(rename = "pausedUP")]
    PausedUploading,
    /// Queuing is enabled and torrent is queued for upload
    #[serde(rename = "queuedUP")]
    QueuedUploading,
    /// Torrent has finished downloading and is being checked
    #[serde(rename = "checkingUP")]
    CheckingUploading,
    /// Torrent is forced to uploading and ignore queue limit
    #[serde(rename = "forcedUP")]
    ForcedUploading,

    // Downloading states
    /// Torrent is being downloaded and data is being transferred
    Downloading,
    /// Torrent has just started downloading and is fetching metadata
    #[serde(rename = "metaDL")]
    MetaDl,
    /// Torrent is paused and has NOT finished downloading
    #[serde(rename = "pausedDL")]
    PausedDownloading,
    /// Queuing is enabled and torrent is queued for download
    #[serde(rename = "queuedDL")]
    QueuedDownloading,
    /// Torrent is being downloaded, but no connection were made
    #[serde(rename = "stalledDL")]
    StalledDownloading,
    /// Same as checkingUP, but torrent has NOT finished downloading
    #[serde(rename = "checkingDL")]
    CheckingDownloading,
    /// Torrent is forced to downloading to ignore queue limit
    #[serde(rename = "forcedDL")]
    ForcedDownloading,

    // Other states
    /// Torrent is allocating disk space for download
    Allocating,
    /// Checking resume data on qBt startup
    #[serde(rename = "checkingResumeData")]
    CheckingResumeData,
    /// Torrent is moving to another location
    Moving,
    /// Unknown status
    Unknown,

    // Completion states
    /// Torrent has completed downloading
    Completed,
    /// Torrent has expired (no longer active)
    Expired,
    /// Torrent is cached
    Cached,

    // Legacy/alias states (for backward compatibility)
    /// Torrent is stalled with no seeds (legacy name)
    #[serde(rename = "stalled (no seeds)", alias = "stalled")]
    StalledNoSeeds,
    /// Torrent is paused (generic paused state)
    #[serde(alias = "paused")]
    Paused,
}
