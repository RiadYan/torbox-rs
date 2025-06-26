use reqwest::Method;
use specta::Type;
use torbox_core_rs::{api::ApiResponse, client::TorboxClient};

use crate::body::{TorrentCreateBody, TorrentInfoByHashBody};

#[derive(Type)]
pub struct TorrentApi<'a> {
    client: &'a TorboxClient,
}

impl<'a> TorrentApi<'a> {
    /// Creates a new torrent on the server using a magnet link or a file URL, not both.
    ///
    /// # Arguments
    ///
    /// * `data` - The payload containing either a magnet link or file path.
    ///
    /// # Returns
    ///
    /// A deserialized `ApiResponse` with the result of the torrent creation.
    pub async fn create_torrent(
        &self,
        data: TorrentCreateBody<'a>,
    ) -> Result<ApiResponse, reqwest::Error> {
        self.client
            .request_with_body(Method::POST, "api/torrent/create", data)
            .await
    }

    /// Retrieves a list of torrents associated with the authenticated user.
    ///
    /// # Returns
    ///
    /// A deserialized `ApiResponse` containing the list of torrents.
    pub async fn list_torrents(&self) -> Result<ApiResponse, reqwest::Error> {
        self.client
            .request(Method::GET, "api/torrents/mylist")
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
    /// A deserialized `ApiResponse` containing metadata about the torrent.
    pub async fn get_torrent_info(
        &self,
        hash: &'a str,
        timeout: Option<u32>,
    ) -> Result<ApiResponse, reqwest::Error> {
        let timeout = timeout.unwrap_or(10);
        let endpoint = format!("api/torrents/torrentinfo?hash={}&timeout={}", hash, timeout);
        self.client.request(Method::GET, &endpoint).await
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
        body: TorrentInfoByHashBody<'a>,
    ) -> Result<ApiResponse, reqwest::Error> {
        self.client
            .request_with_body(Method::POST, "api/torrents/torrentinfo", body)
            .await
    }
}
