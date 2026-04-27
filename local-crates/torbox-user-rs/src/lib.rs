use torbox_core_rs::{
    api::ApiResponse,
    client::{Endpoint, TorboxClient},
    data::user::{
        DeviceCodeAuth, ReferralData, SearchEngineData, SessionToken, SubscriptionData,
        TransactionData, UserProfile,
    },
    error::ApiError,
};

use crate::{
    body::{RefreshApiTokenBody, SearchEngineControlBody, SearchEngineEditBody},
    endpoint::{
        AddReferalPostEp, DeviceCodeAuthGetEp, GetConfirmationCodeGetEp, GetUserDataGetEp,
        ReferralDataGetEp, RefreshApiTokenPostEp, SearchEngineControlPostEp, SearchEngineDataGetEp,
        SearchEngineEditPostEp, SubscriptionDataGetEp, TransactionDataGetEp,
    },
    query::{AddReferralQuery, DeviceCodeAuthQuery, SearchEngineDataQuery, UserDataQuery},
};

pub mod body;
pub mod endpoint;
pub mod query;
pub mod tests;
pub mod types;

/// Main interface for TorBox User operations
///
/// Provides methods for all general API calls including:
/// - Retrieving user data
/// - Adding referal links
/// - Refreshing APIs
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserApi<'a> {
    client: &'a TorboxClient,
}

impl<'a> UserApi<'a> {
    pub fn new(client: &'a TorboxClient) -> Self {
        Self { client }
    }

    /// Not done
    pub async fn refresh_api_token(
        &self,
        token: String,
        expires_at: u64,
    ) -> Result<ApiResponse<String>, ApiError> {
        Endpoint::<RefreshApiTokenPostEp>::new(self.client)
            .call_json(RefreshApiTokenBody {
                session_token: SessionToken { token, expires_at },
            })
            .await
    }

    ///
    /// ## Overview
    ///  Gets a users account data and information.
    ///
    pub async fn get_data(
        &self,
        enable_settings: bool,
    ) -> Result<ApiResponse<UserProfile>, ApiError> {
        Endpoint::<GetUserDataGetEp>::new(self.client)
            .call_query(UserDataQuery {
                settings: enable_settings,
            })
            .await
    }

    pub async fn add_referral_code(&self, referral: String) -> Result<ApiResponse<()>, ApiError> {
        Endpoint::<AddReferalPostEp>::new(self.client)
            .call_query(AddReferralQuery { referral })
            .await
    }

    pub async fn get_confirmation_code(&self) -> Result<ApiResponse<()>, ApiError> {
        Endpoint::<GetConfirmationCodeGetEp>::new(self.client)
            .call_query(())
            .await
    }

    pub async fn start_device_code_auth(
        &self,
        app_name: String,
    ) -> Result<ApiResponse<DeviceCodeAuth>, ApiError> {
        Endpoint::<DeviceCodeAuthGetEp>::new(self.client)
            .call_query(DeviceCodeAuthQuery { app: app_name })
            .await
    }

    pub async fn get_referral_data(&self) -> Result<ApiResponse<ReferralData>, ApiError> {
        Endpoint::<ReferralDataGetEp>::new(self.client)
            .call_query(())
            .await
    }

    pub async fn get_subscription(&self) -> Result<ApiResponse<SubscriptionData>, ApiError> {
        Endpoint::<SubscriptionDataGetEp>::new(self.client)
            .call_query(())
            .await
    }

    pub async fn get_transactions(&self) -> Result<ApiResponse<Vec<TransactionData>>, ApiError> {
        Endpoint::<TransactionDataGetEp>::new(self.client)
            .call_query(())
            .await
    }

    pub async fn get_search_engines(
        &self,
        id: u32,
    ) -> Result<ApiResponse<Vec<SearchEngineData>>, ApiError> {
        Endpoint::<SearchEngineDataGetEp>::new(self.client)
            .call_query(SearchEngineDataQuery { id })
            .await
    }

    pub async fn control_search_engines(
        &self,
        body: SearchEngineControlBody,
    ) -> Result<ApiResponse<()>, ApiError> {
        Endpoint::<SearchEngineControlPostEp>::new(self.client)
            .call_json(body)
            .await
    }

    pub async fn edit_search_engines(
        &self,
        body: SearchEngineEditBody,
    ) -> Result<ApiResponse<SearchEngineData>, ApiError> {
        Endpoint::<SearchEngineEditPostEp>::new(self.client)
            .call_json(body)
            .await
    }
}
