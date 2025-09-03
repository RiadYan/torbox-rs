use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum WebDownloadControlSource {
    /// the torrent's id, optional if using "all" parameter
    WebdlId(u32),
    /// if you want to do this operation to all torrents in your account, optional if using the "torrent_id" parameter
    All(bool),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum WebDownloadOperation {
    Delete,
}
