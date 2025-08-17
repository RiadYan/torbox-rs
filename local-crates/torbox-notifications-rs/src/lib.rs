use torbox_core_rs::{
    api::ApiResponse,
    client::{Endpoint, TorboxClient},
    data::notifications::NotificationFeed,
    error::ApiError,
};

use crate::endpoint::{
    ClearAllNotificationsEp, ClearSingleNotificationEp, GetNotifFeedEp, GetRssNotifFeedEp,
    SendTestNotificationEp,
};

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

    pub async fn get_rss_notif_feed(&self) -> Result<String, ApiError> {
        Endpoint::<GetRssNotifFeedEp>::new(self.client)
            .call_query_raw(self.client.token().to_string())
            .await
    }

    pub async fn get_notif_feed(&self) -> Result<ApiResponse<Vec<NotificationFeed>>, ApiError> {
        Endpoint::<GetNotifFeedEp>::new(self.client)
            .call_query(())
            .await
    }

    pub async fn clear_notification(&self, id: u64) -> Result<ApiResponse<()>, ApiError> {
        Endpoint::<ClearSingleNotificationEp>::new(self.client)
            .call_query(id)
            .await
    }

    pub async fn clear_all_notifications(&self) -> Result<ApiResponse<()>, ApiError> {
        Endpoint::<ClearAllNotificationsEp>::new(self.client)
            .call_query(())
            .await
    }

    pub async fn send_test_notification(&self) -> Result<ApiResponse<()>, ApiError> {
        Endpoint::<SendTestNotificationEp>::new(self.client)
            .call_query(())
            .await
    }
}
