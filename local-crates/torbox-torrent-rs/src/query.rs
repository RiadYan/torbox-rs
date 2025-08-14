use serde::Serialize;

use crate::types::TorrentExportType;

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
    pub bypass_cache: bool,

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

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentRequestLinkQuery {
    /// Hidden and not shown by specta because token is needed for queries here :(
    #[specta(skip)]
    pub(crate) token: String,

    /// The torrent's ID that you want to download
    pub torrent_id: u32,

    /// The files's ID that you want to download.
    pub file_id: Option<u32>,

    /// If you want a zip link. Required if no file_id. Takes precedence over file_id if both are given.
    pub zip_link: bool,

    /// The user's IP to determine the closest CDN. Optional.
    ///
    /// Preferably check IPv4 if correct first.
    pub user_ip: Option<String>,

    /// If you want to redirect the user to the CDN link.
    ///
    /// This is useful for creating permalinks so that you can just make this request URL the link.
    pub redirect: bool,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentExportDataQuery {
    pub torrent_id: u32,
    /// Either "magnet" or "file". Tells how the API what to get, and what to respond as.
    ///
    /// If magnet, it returns a JSON response with the magnet as a string in the data key. If file, it responds with a torrent file download.
    #[serde(rename = "type")]
    pub data_type: TorrentExportType,
}
