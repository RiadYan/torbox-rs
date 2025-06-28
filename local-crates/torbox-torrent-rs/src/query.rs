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

    /// Determines the offset of items to get from the database.
    ///
    /// Default is 0. Optional.
    pub offset: Option<u32>,

    /// Determines the number of items to recieve per request.
    ///
    /// Default is 1000. Optional.
    pub limit: Option<u32>,
}

impl Default for ListTorrentsQuery {
    fn default() -> Self {
        Self {
            bypass_cache: Some(false),
            offset: Some(0),
            limit: Some(1000),
        }
    }
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
    /// Hash of the torrent you want to get info for. This is required.
    pub hash: String,

    /// The amount of time you want TorBox to search for the torrent on the Bittorrent network.
    ///
    /// If the number of seeders is low or even zero, this value may be helpful to move up.
    ///
    /// Default is 10. Optional.
    pub timeout: Option<u32>,
}
