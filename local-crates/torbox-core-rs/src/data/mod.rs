pub mod torrent;
pub mod user;
use crate::data::{
    torrent::{TorrentData, TorrentMeta, TorrentStatus},
    user::{Job, UserProfile},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(untagged)]
pub enum ApiDataResponse {
    Message(String),
    Torrent(Box<TorrentData>),
    Status(Box<Vec<TorrentStatus>>),
    Meta(Box<TorrentMeta>),
    UserData(Box<UserProfile>),
    Jobs(Box<Vec<Job>>),
}
