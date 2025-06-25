pub mod creation;
pub mod limits;
pub mod torrent;
pub mod user;
pub mod webdownload;

use crate::data::{
    creation::TorrentCreationResponse,
    limits::{ActiveLimitStatus, CooldownLimitStatus, MonthlyLimitStatus},
    torrent::{TorrentData, TorrentMap, TorrentMeta, TorrentStatus},
    user::{Job, UserProfile},
    webdownload::{HosterInfo, WebDownloadCreationResponse, WebDownloadStatus},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(untagged)]
pub enum ApiDataResponse {
    Message(String),
    TorrentCache(Box<TorrentData>),
    TorrentStatusList(Box<Vec<TorrentStatus>>),
    TorrentMeta(Box<TorrentMeta>),
    TorrentMap(TorrentMap),
    UserData(Box<UserProfile>),
    Jobs(Box<Vec<Job>>),
    TorrentCreation(TorrentCreationResponse),
    ActiveLimit(ActiveLimitStatus),
    MonthlyLimit(MonthlyLimitStatus),
    CooldownLimit(CooldownLimitStatus),
    WebDownloadCreation(WebDownloadCreationResponse),
    WebDownloadList(Box<Vec<WebDownloadStatus>>),
    HosterInfoList(Box<Vec<HosterInfo>>),
}
