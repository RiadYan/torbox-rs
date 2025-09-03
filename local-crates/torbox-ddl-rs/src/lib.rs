use torbox_core_rs::{
    api::ApiResponse,
    client::{Endpoint, EndpointSpec, TorboxClient},
    data::{
        creation::DownloadLinkResponse,
        webdownload::{WebdownloadCreationResponse, WebdownloadStatus},
    },
    enums::OneOrMany,
    error::ApiError,
};

use crate::{
    body::{WebdownloadControlReq, WebdownloadCreateBody},
    endpoint::{
        ListWebdownloadsGetEp, WebdownloadControlPostEp, WebdownloadCreatePostEp,
        WebdownloadRequestLinkGetEp,
    },
    query::{ListWebdownloadsQuery, WebdownloadRequestLinkQuery},
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

    /// Requests a download link for a torrent
    ///
    /// Links are valid for 3 hours. Once downloading starts, the transfer
    /// can continue indefinitely. Permalinks can and should be created by setting
    /// `redirect=true`.
    ///
    /// Setting `redirect = true` truly helps torbox servers not to be overwhelmed.
    ///
    /// # Arguments
    ///
    /// * `query` - Contains torrent ID and download options
    ///
    /// # Returns
    ///
    /// Either a JSON response or redirect URL
    pub async fn request_download_link(
        &self,
        query: WebdownloadRequestLinkQuery,
    ) -> Result<DownloadLinkResponse, ApiError> {
        let endpoint = format!(
            "{}/{}",
            self.client.base_url,
            WebdownloadRequestLinkGetEp::PATH
        );
        let request = self.client.client.get(&endpoint).query(&query);

        let response = request.send().await?;

        if query.redirect {
            if response.status().is_redirection() {
                let location = response
                    .headers()
                    .get("Location")
                    .ok_or(ApiError::RedirectError("Missing Location header".into()))?
                    .to_str()
                    .map_err(|_| ApiError::RedirectError("Invalid Location header".into()))?;

                Ok(DownloadLinkResponse::Redirect(location.to_string()))
            } else {
                match response.json::<ApiResponse<String>>().await {
                    Ok(json) => Ok(DownloadLinkResponse::Json(json)),
                    Err(_) => Err(ApiError::UnexpectedPayload),
                }
            }
        } else {
            let json = response.json::<ApiResponse<String>>().await?;
            Ok(DownloadLinkResponse::Json(json))
        }
    }

    /// Gets the user's torrent list. This gives you the needed information to perform other torrent actions.
    ///
    /// This information only gets updated every 600 seconds, or when the _Request Update On Torrent request_ is sent to the relay API.
    ///
    /// # Returns
    ///
    /// A deserialized `ApiResponse` containing the list of torrents.
    pub async fn list_webdownloads_query(
        &self,
        query: ListWebdownloadsQuery,
    ) -> Result<ApiResponse<Option<Vec<WebdownloadStatus>>>, ApiError> {
        let resp: ApiResponse<Option<OneOrMany<WebdownloadStatus>>> =
            Endpoint::<ListWebdownloadsGetEp>::new(self.client)
                .call_query(query)
                .await?;

        let normalized = resp.map(|opt| {
            opt.map(|one_or_many| match one_or_many {
                OneOrMany::One(item) => vec![item],
                OneOrMany::Many(list) => list,
            })
        });

        Ok(normalized)
    }
}
