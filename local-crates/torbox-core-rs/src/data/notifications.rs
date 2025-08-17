use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct NotificationFeed {
    pub id: u64,
    pub created_at: DateTime<FixedOffset>,
    pub title: String,
    pub message: String,
    pub auth_id: String,
}
