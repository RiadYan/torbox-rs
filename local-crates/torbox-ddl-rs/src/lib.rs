use torbox_core_rs::{
    api::ApiResponse,
    client::{Endpoint, EndpointSpec, TorboxClient},
    data::{
        creation::DownloadLinkResponse,
        webdownload::{
            WebDownloadCacheAvailability, WebdownloadCreationResponse, WebdownloadHosterList,
            WebdownloadStatus,
        },
    },
    enums::OneOrMany,
    error::ApiError,
};

use crate::{
    body::{WebdownloadControlReq, WebdownloadCreateBody},
    endpoint::{
        ListWebdownloadsGetEp, WebdownloadCachedAvailabilityGetEp, WebdownloadControlPostEp,
        WebdownloadCreatePostEp, WebdownloadHosterListGetEp, WebdownloadRequestLinkGetEp,
    },
    query::{
        ListWebdownloadsQuery, WebdownloadCachedAvailabilityQuery, WebdownloadRequestLinkQuery,
    },
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

    pub async fn create(
        &self,
        body: WebdownloadCreateBody,
    ) -> Result<ApiResponse<WebdownloadCreationResponse>, ApiError> {
        Endpoint::<WebdownloadCreatePostEp>::new(self.client)
            .call_multipart(body)
            .await
    }

    pub async fn control(&self, req: WebdownloadControlReq) -> Result<ApiResponse<()>, ApiError> {
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
    pub async fn list_query(
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

    pub async fn is_cached(
        &self,
        query: WebdownloadCachedAvailabilityQuery,
    ) -> Result<ApiResponse<Option<Vec<WebDownloadCacheAvailability>>>, ApiError> {
        let resp = Endpoint::<WebdownloadCachedAvailabilityGetEp>::new(self.client)
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

    /// ## Overview
    /// A dynamic list of hosters that TorBox is capable of downloading through its paid service.
    /// - Name - a clean name for display use, the well known name of the service, should be recognizable to users.
    /// - Domains - an array of known domains that the hoster uses. While each may serve a different purpose it is still included.
    /// - URL - the main url of the service. This should take you to the home page or a service page of the hoster.
    /// - Icon - a square image, usually a favicon or logo, that represents the service, should be recognizable as the hoster's icon.
    /// - Status - whether this hoster can be used on TorBox or not at the current time. It is usually a good idea to check this value before submitting a download to TorBox's servers for download.
    /// - Type - values are either "hoster" or "stream". Both do the same thing, but is good to differentiate services used for different things.
    /// - Note - a string value (or null) that may give helpful information about the current status or state of a hoster. This can and should be shown to end users.
    /// - Daily Link Limit - the number of downloads a user can use per day. As a user submits links, once they hit this number, the API will deny them from adding anymore of this type of link. A zero value means that it is unlimited.
    /// - Daily Link Used - the number of downloads a user has already used. Usually zero unless you send authentication to the endpoint. This will return accurate values.
    /// - Daily Bandwidth Limit - the value in bytes that a user is allowed to download from this hoster. A zero value means that it is unlimited. It is recommended to use the Daily Link Limit instead.
    /// - Daily Bandwidth Used - the value in bytes that a user has already used to download from this hoster. Usually zero unless you send authentication to the endpoint. This will return accurate values.
    ///
    /// ## Authorization
    /// Optional authorization. Authorization is not required in this endpoint unless you want to get the user's live data. Requires an API key using the Authorization Bearer Header to get the live and accurate data for Daily Link Used and Daily Bandwidth Used
    pub async fn list_hosters(&self) -> Result<ApiResponse<Vec<WebdownloadHosterList>>, ApiError> {
        Endpoint::<WebdownloadHosterListGetEp>::new(self.client)
            .call_query(())
            .await
    }

    /// ## TODO:
    /// This function hasn't yet been made due to laziness.
    ///
    /// ## DANGEROUS
    ///
    /// ## Overview
    /// Updates a download item based on the item's ID.
    /// Sending a PUT request with valid items overwrites the previous items entirely, so use this endpoint with caution. Please make sure to read the restrictions below as they are important.
    ///
    /// ## Restrictions
    /// - Item must be cached to update.
    /// - Does not affect the cached database (or any other user's item).
    /// - Name must pass validation:
    ///     - Above 1 character long.
    ///     - Below 200 characters long.
    ///     - Cannot contain any non-url or non filesystem safe characters. (This is removed automatically).
    ///     - Cannot contain any leading or trailing whitespace. You can have spaces in the middle. (This is removed automatically).
    ///
    /// - Tags follow the same rules as the name.
    /// - Alternative hashes must be of MD5, SHA1, or SHA256.
    pub async fn edit_item_put(&self) {}
}
