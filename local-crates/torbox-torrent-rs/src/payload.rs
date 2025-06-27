use serde::Deserialize;
use torbox_core_rs::data::{
    creation::TorrentCreationResponse,
    limits::{ActiveLimitStatus, CooldownLimitStatus, MonthlyLimitStatus},
    torrent::TorrentMeta,
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TorrentInfoPayload {
    Meta(Box<TorrentMeta>),
    Message(String),
    Empty,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TorrentCreatePayload {
    Created(TorrentCreationResponse),
    ActiveLimit(ActiveLimitStatus),
    MonthlyLimit(MonthlyLimitStatus),
    CooldownLimit(CooldownLimitStatus),
    Message(String),
    Empty,
}
