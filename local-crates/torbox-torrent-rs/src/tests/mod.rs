#[cfg(test)]
mod torrent_test {
    use std::env;

    use crate::{
        body::{TorrentCreateBody, TorrentInfoBody},
        client::TorrentApi,
        query::{ListTorrentsQuery, TorrentInfoQuery, TorrentStatusQuery},
        types::TorrentSource,
    };

    use dotenvy::from_filename;
    use torbox_core_rs::{body::ToMultipart, client::TorboxClient};

    pub fn load_token_from_file() -> Option<String> {
        if let Err(err) = from_filename(".token") {
            eprintln!(
                "Could not load .token file, please create one before starting unit testing: {err}"
            );
            return None;
        }
        env::var("TORBOX_TOKEN").ok()
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

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_list_torrents_with_query_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let query = ListTorrentsQuery::default();
        let result = api.list_torrents_with_query(query).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_torrent_status_query_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let query = TorrentStatusQuery {
            bypass_cache_id: true,
            id: 2569352,
        };
        let result = api.torrent_status_query(query).await;

        match result {
            Ok(response) => {
                println!("Success: {:?}", response);
            }
            Err(e) => {
                eprintln!("Error during create_torrent: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_torrent_info_query_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let query = TorrentInfoQuery {
            hash: "fbf9f51b5e67bb9612ced592868d6704300584f4".to_string(),
            timeout: Some(5),
        };

        let result = api.torrent_info_query(query).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_torrent_info_body_success() {
        let client = test_client();
        let api = TorrentApi::new(&client);

        let body = TorrentInfoBody::try_new(
            Some("fbf9f51b5e67bb9612ced592868d6704300584f4".to_string()),
            None,
            None,
            Some(5),
        )
        .unwrap();

        let result = api.torrent_info_body(body).await;
        match result {
            Ok(response) => {
                println!("Success: {:?}", response);
            }
            Err(e) => {
                eprintln!("Error during list_torrent: {e:?}");
            }
        }
    }
}
