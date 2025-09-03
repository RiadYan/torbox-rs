#[cfg(test)]
pub mod torrent_test {
    use std::env;

    use crate::{
        TorrentApi,
        body::{TorrentControlBody, TorrentCreateBody, TorrentInfoBody},
        query::{
            ListTorrentsQuery, TorrentExportDataQuery, TorrentInfoQuery, TorrentRequestLinkQuery,
            TorrentStatusQuery,
        },
        types::{TorrentDownloadResponse, TorrentExportResponse, TorrentExportType, TorrentSource},
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
            add_only_if_cached: None
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
                println!("Success: {:?}", response);
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

        let result = api.status_query(true, get_first_torrent_id().await).await;

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

        let result = api
            .info_query(get_first_torrent_hash().await, Some(5))
            .await;
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
            files_id: None,
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

    #[tokio::test]
    async fn test_torrent_export_data_query_magnet_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        match api
            .export_data_query(get_first_torrent_id().await, TorrentExportType::Magnet)
            .await
        {
            Ok(TorrentExportResponse::Json(response)) => {
                println!("Magnet URI: {:?}", response.data);
                assert!(response.success);
                assert!(response.data.unwrap().starts_with("magnet:"));
            }
            Ok(_) => panic!("Expected JSON response but got file"),
            Err(e) => panic!("API call failed: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_torrent_export_data_query_file_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        match api
            .export_data_query(get_first_torrent_id().await, TorrentExportType::File)
            .await
        {
            Ok(TorrentExportResponse::File(data)) => {
                println!("Received file with {} bytes", data.len());
                assert!(!data.is_empty());
                assert!(data.starts_with(b"d8:announce"), "Issue: {data:?}"); // torrent file validation
            }
            Ok(_) => panic!("Expected file response but got JSON"),
            Err(e) => panic!("API call failed: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_torrent_control_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let body = TorrentControlBody {
            source: crate::types::TorrentControlSource::TorrentId(get_first_torrent_id().await),
            operation: crate::types::TorrentOperation::Resume,
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
