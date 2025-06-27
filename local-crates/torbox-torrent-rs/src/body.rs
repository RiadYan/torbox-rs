use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TorrentSource {
    Magnet(String),
    File(String),
}

/// Request body for retrieving torrent information from TorBox.
///
/// This struct allows you to pass one of the following fields (in order of precedence):
/// - `hash` (highest precedence)
/// - `magnet` link
/// - `file` (base64 string)
///
/// At least one of these must be provided, or the request will be considered invalid.
///
/// Use the [`try_new`](Self::try_new) constructor to safely build this struct.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentInfoBody {
    /// The torrent hash to search for (takes highest precedence).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,

    /// A magnet link, used if `hash` is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnet: Option<String>,

    /// A torrent file string base64, used if neither `hash` nor `magnet` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    /// Timeout in seconds for the lookup. Defaults to 10.
    #[serde(default)]
    pub timeout: u32,
}

impl TorrentInfoBody {
    /// Creates a `TorrentInfoByHashBody` with correct precedence enforcement.
    ///
    /// # Precedence
    /// - `hash` is used if present
    /// - else `magnet`
    /// - else `file`
    ///
    /// # Returns
    ///
    /// Returns `Some(Self)` if at least one of the three main fields is provided. Otherwise, returns `None`.
    pub fn try_new(
        hash: Option<String>,
        magnet: Option<String>,
        file: Option<String>,
        timeout: Option<u32>,
    ) -> Option<Self> {
        match (hash, magnet, file) {
            (Some(hash), _, _) => Some(Self {
                hash: Some(hash),
                magnet: None,
                file: None,
                timeout: timeout.unwrap_or(10),
            }),
            (None, Some(magnet), _) => Some(Self {
                hash: None,
                magnet: Some(magnet),
                file: None,
                timeout: timeout.unwrap_or(10),
            }),
            (None, None, Some(file)) => Some(Self {
                hash: None,
                magnet: None,
                file: Some(file),
                timeout: timeout.unwrap_or(10),
            }),
            (None, None, None) => None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentCreateBody {
    #[serde(flatten)]
    pub source: TorrentSource,

    /// Tells TorBox your preference for seeding this torrent.
    /// 1 is auto. 2 is seed. 3 is don't seed.
    ///
    /// Optional. Default is 1, or whatever the user has in their settings. Overwrites option in settings.
    pub seed: Option<u8>,

    /// Tells TorBox if you want to allow this torrent to be zipped or not.
    ///
    /// TorBox only zips if the torrent is 100 files or larger.
    pub allow_zip: bool,

    /// The name you want the torrent to be. Optional.
    pub name: Option<String>,

    /// Tells TorBox you want this torrent instantly queued.
    ///
    /// This is **bypassed** if user is on free plan, and will process the request as normal in this case. Optional.
    pub queued: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentListBody {
    /// Allows you to bypass the cached data, and always get fresh information.
    ///
    /// Useful if constantly querying for fresh download stats.
    /// Otherwise, we request that you save our database a few calls.
    pub bypass_cache_id: bool,

    /// Determines the torrent requested, will return an object rather than list. Optional.
    pub id: Option<u32>,

    /// Determines the offset of items to get from the database. Default is 0. Optional.
    pub offset: Option<u32>,

    /// Determines the number of items to recieve per request. Default is 1000. Optional.
    pub limit: Option<u32>,
}
