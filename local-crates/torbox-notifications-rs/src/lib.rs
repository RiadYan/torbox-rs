use torbox_core_rs::{
    client::{Endpoint, TorboxClient},
    error::ApiError,
};

use crate::endpoint::GetRssNotifFeedEp;

pub mod endpoint;
pub mod query;
pub mod tests;
pub mod types;

/// Main interface for TorBox general operations
///
/// Provides methods for all general API calls including:
/// - Up status
/// - Changelogs in both RSS and JSON
/// - Test files
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct NotificationApi<'a> {
    client: &'a TorboxClient,
}

impl<'a> NotificationApi<'a> {
    pub fn new(client: &'a TorboxClient) -> Self {
        Self { client }
    }

    pub async fn get_changelog_rss_feed(&self) -> Result<String, ApiError> {
        Endpoint::<GetRssNotifFeedEp>::new(self.client)
            .call_query_raw(self.client.token().to_string())
            .await
    }
}
