use serde::Serialize;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadRequestLinkQuery {
    /// Hidden and not shown by specta because token is needed for queries here :(
    #[cfg_attr(feature = "specta", specta(skip))]
    pub(crate) token: String,

    /// The torrent's ID that you want to download
    pub web_id: u32,

    /// The files's ID that you want to download.
    pub files_id: Option<Vec<u32>>,

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
pub struct WebdownloadControlQuery {
    pub bypass_cache: bool,
}

/// `id` param isn't given because if it is, it will return an Object and not a Vec
///
/// Please use TorrentStatusQuery instead
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ListWebdownloadsQuery {
    id: Option<u32>,
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
