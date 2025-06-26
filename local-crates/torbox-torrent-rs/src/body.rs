use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum TorrentCreateBody<'a> {
    Magnet(&'a str),
    File(&'a str),
}

/// Request body for retrieving torrent information from TorBox.
///
/// This struct allows you to pass one of the following in order of precedence:
/// - A `hash` (highest precedence)
/// - A `magnet` link
/// - A `file` (must be a valid torrent file path or base64-encoded string depending on API expectation)
///
/// At least one of these must be provided, or the request will be considered invalid.
///
/// Use the [`try_new`](TorrentInfoByHashBody::try_new) constructor to enforce this logic safely
#[derive(Debug, Serialize)]
pub struct TorrentInfoByHashBody<'a> {
    /// The torrent hash to search for (takes highest precedence).
    #[serde(skip_serializing_if = "Option::is_none")]
    hash: Option<&'a str>,

    /// Timeout in seconds for the torrent lookup. Defaults to 10.
    #[serde(default)]
    timeout: u32,

    /// The source of the torrent, either a magnet link or a torrent file.
    ///
    /// This is only used if `hash` is not provided. If both magnet and file are provided,
    /// the magnet will be used (takes precedence over file).
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    source: Option<TorrentCreateBody<'a>>,
}

impl<'a> TorrentInfoByHashBody<'a> {
    /// Creates a `TorrentInfoByHashBody` with proper field precedence.
    ///
    /// # Precedence
    /// - `hash` is used if present
    /// - else `magnet`
    /// - else `file`
    ///
    /// At least one of the above must be provided or this will return `None`.
    pub fn try_new(
        hash: Option<&'a str>,
        magnet: Option<&'a str>,
        file: Option<&'a str>,
        timeout: Option<u32>,
    ) -> Option<Self> {
        if hash.is_none() && magnet.is_none() && file.is_none() {
            return None;
        }

        let source = match (hash, magnet, file) {
            (Some(_), _, _) => None,
            (None, Some(magnet), _) => Some(TorrentCreateBody::Magnet(magnet)),
            (None, None, Some(file)) => Some(TorrentCreateBody::File(file)),
            _ => None,
        };

        Some(Self {
            hash,
            timeout: timeout.unwrap_or(10),
            source,
        })
    }
}
