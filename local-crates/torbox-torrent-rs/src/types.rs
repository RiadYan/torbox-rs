use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TorrentSource {
    Magnet(String),
    File(PathBuf),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TorrentOperation {
    /// Reannounces the torrent to get new peers
    Reannounce,
    /// Deletes the torrent from the client and your account permanently
    Delete,
    /// Resumes a paused torrent
    Resume,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TorrentControlSource {
    /// the torrent's id, optional if using "all" parameter
    TorrentId(u32),
    /// if you want to do this operation to all torrents in your account, optional if using the "torrent_id" parameter
    All(bool),
}
