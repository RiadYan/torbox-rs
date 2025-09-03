use torbox_core_rs::{
    api::ApiResponse,
    client::{Endpoint, TorboxClient},
    data::webdownload::WebdownloadCreationResponse,
    error::ApiError,
};

use crate::{
    body::{WebdownloadControlReq, WebdownloadCreateBody},
    endpoint::{WebdownloadControlPostEp, WebdownloadCreatePostEp},
};

//todo: Add the rest for ddl
pub mod body;
pub mod endpoint;
pub mod payload;
pub mod query;
pub mod tests;
pub mod types;

#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadApi<'a> {
    client: &'a TorboxClient,
}

impl<'a> WebdownloadApi<'a> {
    pub fn new(client: &'a TorboxClient) -> Self {
        Self { client }
    }

    pub async fn create_webdownload(
        &self,
        body: WebdownloadCreateBody,
    ) -> Result<ApiResponse<WebdownloadCreationResponse>, ApiError> {
        Endpoint::<WebdownloadCreatePostEp>::new(self.client)
            .call_multipart(body)
            .await
    }

    pub async fn control_webdownload(
        &self,
        req: WebdownloadControlReq,
    ) -> Result<ApiResponse<()>, ApiError> {
        let (query, body) = req.into_parts();

        Endpoint::<WebdownloadControlPostEp>::new(self.client)
            .call_query_json(query, body)
            .await
    }
}
