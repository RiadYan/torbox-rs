#[cfg(test)]
mod torrent_test {
    use std::env;

    use crate::{
        body::{TorrentControlBody, TorrentCreateBody, TorrentInfoBody},
        client::TorrentApi,
        query::{ListTorrentsQuery, TorrentInfoQuery, TorrentRequestLinkQuery, TorrentStatusQuery},
        types::{TorrentDownloadResponse, TorrentSource},
    };

    use dotenvy::from_filename;
    use torbox_core_rs::client::TorboxClient;

    pub fn load_token_from_file() -> Option<String> {
        if let Err(err) = from_filename(".token") {
            eprintln!(
                "Could not load .token file, please create one before starting unit testing: {err}"
            );
            return None;
        }
        env::var("TORBOX_TOKEN").ok()
    }

    async fn get_first_torrent_id() -> u32 {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let query = ListTorrentsQuery::default();

        api.list_torrents_query(query)
            .await
            .unwrap()
            .data
            .flatten()
            .unwrap()[0]
            .id as u32
    }

    async fn get_first_torrent_hash() -> String {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let query = ListTorrentsQuery::default();

        api.list_torrents_query(query)
            .await
            .unwrap()
            .data
            .flatten()
            .unwrap()[0]
            .hash
            .clone()
    }

    fn test_client() -> TorboxClient {
        let token = load_token_from_file().expect("Missing TORBOX_TOKEN in .token file");
        TorboxClient::new(token)
    }

    #[tokio::test]
    async fn test_create_torrent_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let body = TorrentCreateBody {
            source: TorrentSource::Magnet(
                "magnet:?xt=urn:btih:a492f8b92a25b0399c87715fc228c864ac5a7bfb&dn=archlinux-2025.06.01-x86_64.iso".into(),
            ),
            seed: None,
            allow_zip: true,
            name: None,
            as_queued: None,
        };

        let result = api.create_torrent(body).await;

        match result {
            Ok(response) => {
                println!("Success: {response:?}");
                assert!(response.success, "API responded with success = false");
            }
            Err(e) => {
                eprintln!("Error during request_download: {e:?}");
                panic!("API call failed: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_list_torrents_with_query_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let query = ListTorrentsQuery::default();
        let result = api.list_torrents_query(query).await;

        match result {
            Ok(response) => {
                println!("Success: {:?}", response.data.unwrap().unwrap()[0].name);
                assert!(response.success, "API responded with success = false");
            }
            Err(e) => {
                eprintln!("Error during request_download: {e:?}");
                panic!("API call failed: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_torrent_status_query_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let query = TorrentStatusQuery {
            bypass_cache: true,
            id: get_first_torrent_id().await,
        };
        let result = api.status_query(query).await;

        match result {
            Ok(response) => {
                println!("Success: {response:?}");
                assert!(response.success, "API responded with success = false");
            }
            Err(e) => {
                eprintln!("Error during request_download: {e:?}");
                panic!("API call failed: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_torrent_info_query_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let query = TorrentInfoQuery {
            hash: get_first_torrent_hash().await,
            timeout: Some(5),
        };

        let result = api.info_query(query).await;
        match result {
            Ok(response) => {
                println!("Success: {response:?}");
                assert!(response.success, "API responded with success = false");
            }
            Err(e) => {
                eprintln!("Error during request_download: {e:?}");
                panic!("API call failed: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_torrent_info_body_success() {
        let client = test_client();

        let api = TorrentApi::new(&client);

        let body =
            TorrentInfoBody::try_new(Some(get_first_torrent_hash().await), None, None, Some(5))
                .unwrap();

        let result = api.info_body(body).await;
        match result {
            Ok(response) => {
                println!("Success: {response:?}");
                assert!(response.success, "API responded with success = false");
            }
            Err(e) => {
                eprintln!("Error during request_download: {e:?}");
                panic!("API call failed: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_torrent_request_download_link_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let query = TorrentRequestLinkQuery {
            token: api.token().to_string(),
            torrent_id: get_first_torrent_id().await,
            file_id: None,
            zip_link: true,
            user_ip: None,
            redirect: true,
        };

        match api.request_download_link(query).await {
            Ok(response) => match response {
                TorrentDownloadResponse::Json(json) => {
                    println!("JSON Response: {json:?}");
                    assert!(json.success, "API responded with success=false");
                    assert!(
                        json.data.is_some(),
                        "Expected download URL in response data"
                    );
                    assert!(
                        json.data.unwrap().starts_with("http"),
                        "Invalid URL format in response"
                    );
                }
                TorrentDownloadResponse::Redirect(url) => {
                    println!("Redirect URL: {url}");
                    assert!(url.starts_with("http"), "Invalid redirect URL format");
                }
            },
            Err(e) => {
                eprintln!("Error during request: {e:?}");
                panic!("API call failed: {e}");
            }
        }
    }
    /// WARNING: Keep as last
    #[tokio::test]
    async fn test_torrent_control_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let body = TorrentControlBody {
            source: crate::types::TorrentControlSource::TorrentId(get_first_torrent_id().await),
            operation: crate::types::TorrentOperation::Delete,
        };
        let result = api.control_torrent(body).await;
        match result {
            Ok(response) => {
                println!("Success: {response:?}");
                assert!(response.success, "API responded with success = false");
            }
            Err(e) => {
                eprintln!("Error during request_download: {e:?}");
                panic!("API call failed: {e:?}");
            }
        }
    }
}
