use reqwest::Method;
use torbox_core_rs::{
    api::ApiResponse,
    client::{Endpoint, TorboxClient},
    data::general::{ChangelogJsonVersion, SpeedtestFile, TorboxGeneralStats},
    error::ApiError,
};

use crate::{
    endpoint::{
        GetChangelogJsonVersionsEp, GetChangelogRssFeedEp, GetSpeedtestFilesEp, GetStatsEp,
        GetUpStatusEp,
    },
    query::SpeedTestQuery,
};

pub mod body;
pub mod endpoint;
pub mod payload;
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
pub struct GeneralApi<'a> {
    client: &'a TorboxClient,
}

impl<'a> GeneralApi<'a> {
    pub fn new(client: &'a TorboxClient) -> Self {
        Self { client }
    }

    pub async fn get_up_status(&self) -> Result<ApiResponse<()>, ApiError> {
        let tmp_client = self.client.with_base_url("https://api.torbox.app");
        tmp_client.request_with_query(Method::GET, "", &()).await
    }

    pub async fn get_stats(&self) -> Result<ApiResponse<TorboxGeneralStats>, ApiError> {
        Endpoint::<GetStatsEp>::new(self.client)
            .call_query(())
            .await
    }

    pub async fn get_changelog_rss_feed(&self) -> Result<String, ApiError> {
        Endpoint::<GetChangelogRssFeedEp>::new(self.client)
            .call_query_raw(())
            .await
    }

    pub async fn get_changelog_json_versions(
        &self,
    ) -> Result<ApiResponse<Vec<ChangelogJsonVersion>>, ApiError> {
        Endpoint::<GetChangelogJsonVersionsEp>::new(self.client)
            .call_query(())
            .await
    }

    pub async fn get_speedtest_files(
        &self,
        query: SpeedTestQuery,
    ) -> Result<ApiResponse<Vec<SpeedtestFile>>, ApiError> {
        Endpoint::<GetSpeedtestFilesEp>::new(self.client)
            .call_query(query)
            .await
    }
}
