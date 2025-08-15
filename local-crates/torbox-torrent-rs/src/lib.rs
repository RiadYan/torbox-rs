pub mod body;
pub mod endpoint;
pub mod payload;
pub mod query;
pub mod tests;
pub mod types;

use torbox_core_rs::{
    api::ApiResponse,
    client::{Endpoint, EndpointSpec, TorboxClient},
    data::torrent::TorrentStatus,
    error::ApiError,
};

use crate::{
    body::{TorrentControlBody, TorrentCreateBody, TorrentInfoBody},
    endpoint::{
        ListTorrentsGetEp, TorrentControlPostEp, TorrentCreatePostEp, TorrentExportDataGetEp,
        TorrentInfoGetEp, TorrentInfoPostEp, TorrentRequestLinkGetEp, TorrentStatusGetEp,
    },
    payload::{TorrentCreatePayload, TorrentInfoPayload},
    query::{
        ListTorrentsQuery, TorrentExportDataQuery, TorrentInfoQuery, TorrentRequestLinkQuery,
        TorrentStatusQuery,
    },
    types::{TorrentDownloadResponse, TorrentExportResponse, TorrentExportType, TorrentSource},
};

/// Main interface for TorBox torrent operations
///
/// Provides methods for all torrent-related API calls including:
/// - Torrent creation and management
/// - Status and metadata retrieval
/// - Download link generation
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentApi<'a> {
    client: &'a TorboxClient,
}

impl<'a> TorrentApi<'a> {
    pub fn new(client: &'a TorboxClient) -> Self {
        Self { client }
    }

    pub(crate) fn token(&self) -> &str {
        self.client.token()
    }

    /// Creates a torrent under your account
    ///
    /// Accepts either a magnet link or torrent file. The torrent will begin downloading
    /// once verified, assuming your account has available download slots.
    ///
    /// # Arguments
    ///
    /// * `body` - Contains either a magnet link or file path with optional parameters
    ///
    /// # Returns
    ///
    /// An `ApiResponse` with the torrent creation result
    pub async fn create_torrent(
        &self,
        body: TorrentCreateBody,
    ) -> Result<ApiResponse<TorrentCreatePayload>, ApiError> {
        Endpoint::<TorrentCreatePostEp>::new(self.client)
            .call_multipart(body)
            .await
    }

    /// Gets the user's torrent list. This gives you the needed information to perform other torrent actions.
    ///
    /// This information only gets updated every 600 seconds, or when the _Request Update On Torrent request_ is sent to the relay API.
    ///
    /// # Returns
    ///
    /// A deserialized `ApiResponse` containing the list of torrents.
    pub async fn list_torrents_query(
        &self,
        query: ListTorrentsQuery,
    ) -> Result<ApiResponse<Option<Vec<TorrentStatus>>>, ApiError> {
        Endpoint::<ListTorrentsGetEp>::new(self.client)
            .call_query(query)
            .await
    }

    /// Gets detailed status for a specific torrent
    ///
    /// # Arguments
    ///
    /// * `query` - Contains the torrent ID and cache bypass option
    ///
    /// # Returns
    ///
    /// An `ApiResponse` with the torrent's current status
    pub async fn status_query(
        &self,
        bypass_cache: bool,
        id: u32,
    ) -> Result<ApiResponse<Option<TorrentStatus>>, ApiError> {
        Endpoint::<TorrentStatusGetEp>::new(self.client)
            .call_query(TorrentStatusQuery { bypass_cache, id })
            .await
    }

    /// Fetches torrent metadata using a GET request with query parameters.
    ///
    /// This is a general-purpose route that takes a torrent `hash` and queries the BitTorrent network
    /// for information about it. This data retrieval may take time and will timeout by default after
    /// 10 seconds. You may adjust this with the optional `timeout` parameter.
    ///
    /// This route is cached, so repeated requests for the same torrent hash will return instantly
    /// after the first request.
    ///
    /// # Arguments
    ///
    /// * `hash` - The torrent info hash to search for.
    /// * `timeout` - Optional timeout in seconds for the request. Defaults to 10 seconds.
    ///
    /// # Returns
    ///
    /// ## Success payload (`TorrentInfoPayload`)
    /// * `Meta(Box<TorrentMeta>)` – full metadata (name, size, file list, …)  
    /// * `Message(String)`      – informational message (e.g. “not found yet”)
    ///
    /// ## Errors
    /// Network / JSON errors → `ApiError::Transport`  
    /// `success == false`     → `ApiError::Failure`  
    /// Unexpected JSON        → `ApiError::UnexpectedPayload`
    pub async fn info_query(
        &self,
        hash: String,
        timeout: Option<u32>,
    ) -> Result<ApiResponse<TorrentInfoPayload>, ApiError> {
        Endpoint::<TorrentInfoGetEp>::new(self.client)
            .call_query(TorrentInfoQuery { hash, timeout })
            .await
    }

    /// Fetches torrent metadata using a POST request with flexible input types.
    ///
    /// This endpoint allows sending a `hash`, `magnet` link, or a raw torrent file. TorBox will prioritize
    /// the fields in the following order:
    /// - `hash` takes precedence over
    /// - `magnet`, which takes precedence over
    /// - `file`
    ///
    /// At least one of these fields is required. Only valid torrent files are accepted.
    ///
    /// Like the GET route, data is fetched from the BitTorrent network and cached for future requests.
    /// The default timeout is 10 seconds but can be customized.
    ///
    /// # Arguments
    ///
    /// * `body` - A [`TorrentInfoBody`] struct containing `hash`, `timeout`, and the source type.
    ///
    /// # Returns
    ///
    /// A deserialized `ApiResponse` containing metadata about the torrent.
    pub async fn info_body(
        &self,
        body: TorrentInfoBody,
    ) -> Result<ApiResponse<TorrentInfoPayload>, ApiError> {
        Endpoint::<TorrentInfoPostEp>::new(self.client)
            .call_multipart(body)
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
        query: TorrentRequestLinkQuery,
    ) -> Result<TorrentDownloadResponse, ApiError> {
        let endpoint = format!("{}/{}", self.client.base_url, TorrentRequestLinkGetEp::PATH);
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

                Ok(TorrentDownloadResponse::Redirect(location.to_string()))
            } else {
                match response.json::<ApiResponse<String>>().await {
                    Ok(json) => Ok(TorrentDownloadResponse::Json(json)),
                    Err(_) => Err(ApiError::UnexpectedPayload),
                }
            }
        } else {
            let json = response.json::<ApiResponse<String>>().await?;
            Ok(TorrentDownloadResponse::Json(json))
        }
    }

    /// Controls torrent state (start, stop, delete, etc.)
    ///
    /// # Arguments
    ///
    /// * `body` - Contains the torrent source and operation to perform
    ///
    /// # Returns
    ///
    /// An empty `ApiResponse` on success
    pub async fn control_torrent(
        &self,
        body: TorrentControlBody,
    ) -> Result<ApiResponse<()>, ApiError> {
        Endpoint::<TorrentControlPostEp>::new(self.client)
            .call(body)
            .await
    }

    /// Exports the magnet or torrent file.
    ///
    /// Requires a type to be passed. If type is magnet, it will return a JSON response with the magnet as a string in the data key.
    ///
    ///  If type is file, it will return a bittorrent file as a download. Not compatible with cached downloads.
    pub async fn export_data_query(
        &self,
        torrent_id: u32,
        data_type: TorrentExportType,
    ) -> Result<TorrentExportResponse, ApiError> {
        let endpoint = Endpoint::<TorrentExportDataGetEp>::new(self.client);

        match data_type {
            TorrentExportType::File => {
                let bytes = endpoint
                    .call_query_bytes(TorrentExportDataQuery {
                        torrent_id,
                        data_type,
                    })
                    .await?;
                Ok(TorrentExportResponse::File(bytes))
            }
            TorrentExportType::Magnet => {
                let response: ApiResponse<String> = endpoint
                    .call_query(TorrentExportDataQuery {
                        torrent_id,
                        data_type,
                    })
                    .await?;
                Ok(TorrentExportResponse::Json(response))
            }
        }
    }
}
