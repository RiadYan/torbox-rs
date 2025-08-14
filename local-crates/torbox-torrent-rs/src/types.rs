use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use torbox_core_rs::{api::ApiResponse, error::ApiError, traits::FromBytes};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TorrentSource {
    Magnet(String),
    File(Vec<u8>),
}

impl<'de> serde::Deserialize<'de> for TorrentSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = String::deserialize(deserializer)?;

        if val.starts_with("magnet:") {
            Ok(TorrentSource::Magnet(val))
        } else {
            // if it's not a magnet, treat it as base64-encoded file bytes
            let bytes = base64::decode(&val)
                .map_err(|e| serde::de::Error::custom(format!("Invalid base64: {}", e)))?;
            Ok(TorrentSource::File(bytes))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum TorrentDownloadResponse {
    Json(ApiResponse<String>),
    Redirect(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TorrentExportResponse {
    /// JSON response containing API data
    Json(ApiResponse<String>),
    /// Raw file bytes response
    /// TODO: better handling and perf for files
    File(Vec<u8>),
}

impl FromBytes for TorrentExportResponse {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, ApiError> {
        if let Ok(json) = serde_json::from_slice::<ApiResponse<String>>(&bytes) {
            Ok(Self::Json(json))
        } else {
            Ok(Self::File(bytes))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TorrentExportType {
    Magnet,
    File,
}
