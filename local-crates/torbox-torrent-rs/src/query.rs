use serde::Serialize;

/// `id` param isn't given because if it is, it will return an Object and not a Vec
///
/// Please use TorrentStatusQuery instead
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ListTorrentsQuery {
    /// Allows you to bypass the cached data, and always get fresh information.
    ///
    /// Useful if constantly querying for fresh download stats.
    /// Otherwise, we request that you save our database a few calls.
    pub bypass_cache: Option<bool>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentStatusQuery {
    /// Allows you to bypass the cached data, and always get fresh information.
    ///
    /// Useful if constantly querying for fresh download stats.
    /// Otherwise, we request that you save our database a few calls.
    pub bypass_cache_id: bool,

    /// Determines the torrent requested, will return an object rather than list. Not optional.
    pub id: u32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentInfoQuery {
    pub hash: String,
    pub timeout: Option<u32>,
}
