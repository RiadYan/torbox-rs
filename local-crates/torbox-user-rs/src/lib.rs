use torbox_core_rs::{
    api::ApiResponse,
    client::{Endpoint, TorboxClient},
    data::user::{SessionToken, UserProfile},
    error::ApiError,
};

use crate::{
    body::RefreshApiTokenBody,
    endpoint::{GetConfirmationCodeEp, GetUserDataEp, RefreshApiToken},
    query::UserDataQuery,
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
    ///
    /// ## Overview
    ///  Gets a users account data and information.
    ///
    pub async fn get_user_data(
        &self,
        enable_settings: bool,
    ) -> Result<ApiResponse<UserProfile>, ApiError> {
        Endpoint::<GetUserDataEp>::new(self.client)
            .call_query(UserDataQuery {
                settings: enable_settings,
            })
            .await
    }

    pub async fn get_confirmation_code(&self) -> Result<ApiResponse<()>, ApiError> {
        Endpoint::<GetConfirmationCodeEp>::new(self.client)
            .call_query(())
            .await
    }

    /// Not done
    pub(crate) async fn refresh_api_token(
        &self,
        token: String,
        expires_at: u64,
    ) -> Result<ApiResponse<String>, ApiError> {
        Endpoint::<RefreshApiToken>::new(self.client)
            .call(RefreshApiTokenBody {
                session_token: SessionToken { token, expires_at },
            })
            .await
    }
}
