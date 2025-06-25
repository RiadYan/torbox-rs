use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentCreationResponse {
    #[serde(alias = "torrent_id")]
    #[serde(alias = "queued_id")]
    pub id: u64,
    pub auth_id: String,
    pub hash: String,
}
