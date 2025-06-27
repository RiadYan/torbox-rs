use reqwest::Method;
use torbox_core_rs::{
    api::ApiResponse,
    client::{Endpoint, EndpointSpec, TorboxClient},
    data::{ApiDataResponse, torrent::TorrentStatus},
    error::ApiError,
};

use crate::{
    body::{TorrentCreateBody, TorrentInfoBody},
    endpoint::{ListTorrentsGetEp, TorrentCreateEp, TorrentInfoGetEp, TorrentInfoPostEp},
    payload::{TorrentCreatePayload, TorrentInfoPayload},
    query::ListTorrentsQuery,
};

#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorrentApi<'a> {
    client: &'a TorboxClient,
}

impl<'a> TorrentApi<'a> {
    /// Creates a torrent under your account. Simply send either a magnet link, or a torrent file.
    ///
    /// Once they have been checked, they will begin downloading assuming your account has available active download slots, and they aren't too large.  
    ///
    /// # Arguments
    ///
    /// * `body` - The payload containing either a magnet link or file path and optional features.
    ///
    /// # Returns
    ///
    /// A deserialized `ApiResponse` with the result of the torrent creation.
    pub async fn create_torrent(
        &self,
        body: TorrentCreateBody,
    ) -> Result<ApiResponse<TorrentCreatePayload>, ApiError> {
        Endpoint::<TorrentCreateEp>::new(self.client)
            .call(body)
            .await
    }

    /// Gets the user's torrent list. This gives you the needed information to perform other torrent actions.
    ///
    /// This information only gets updated every 600 seconds, or when the _Request Update On Torrent request_ is sent to the relay API.
    ///
    /// # Returns
    ///
    /// A deserialized `ApiResponse` containing the list of torrents.
    pub async fn list_torrents_with_query(
        &self,
        query: ListTorrentsQuery,
    ) -> Result<ApiResponse<Option<Vec<TorrentStatus>>>, ApiError> {
        let endpoint = Endpoint::<ListTorrentsGetEp>::new(self.client);

        endpoint.call_query(query).await
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
    pub async fn get_torrent_info(
        &self,
        hash: &'a str,
        timeout: Option<u32>,
    ) -> Result<ApiResponse<TorrentInfoPayload>, ApiError> {
        let url = format!(
            "{}?hash={}&timeout={}",
            TorrentInfoGetEp::PATH,
            hash,
            timeout.unwrap_or(10)
        );
        Endpoint::<TorrentInfoGetEp>::new(self.client)
            .call_no_body(&url) // Req = ()
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
    /// * `body` - A [`TorrentInfoByHashBody`] struct containing `hash`, `timeout`, and the source type.
    ///
    /// # Returns
    ///
    /// A deserialized `ApiResponse` containing metadata about the torrent.
    pub async fn get_torrent_info_body(
        &self,
        body: TorrentInfoBody,
    ) -> Result<ApiResponse<TorrentInfoPayload>, ApiError> {
        Endpoint::<TorrentInfoPostEp>::new(self.client)
            .call(body)
            .await
    }
}
