use serde::Serialize;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ListTorrentsQuery {
    pub bypass_cache: Option<bool>,
    pub id: Option<u64>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentInfoQuery {
    pub hash: String,
    pub timeout: Option<u32>,
}
