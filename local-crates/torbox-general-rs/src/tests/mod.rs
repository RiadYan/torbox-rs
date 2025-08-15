#[cfg(test)]
pub mod general_test {
    use std::env;

    use crate::{GeneralApi, query::SpeedTestQuery, types::FileLength};

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

    fn test_client() -> TorboxClient {
        let token = load_token_from_file().expect("Missing TORBOX_TOKEN in .token file");
        TorboxClient::new(token)
    }

    #[tokio::test]
    async fn test_get_up_status_success() {
        let client = test_client();
        let api = GeneralApi::new(&client);

        let result = api.get_up_status().await;

        match result {
            Ok(response) => {
                println!("Up Status: {:?}", response);
                assert!(response.success, "API responded with success=false");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_get_stats_success() {
        let client = test_client();
        let api = GeneralApi::new(&client);

        let result = api.get_stats().await;

        match result {
            Ok(response) => {
                println!("Stats: {:?}", response);
                assert!(response.success, "API responded with success=false");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_get_changelog_rss_feed_success() {
        let client = test_client();
        let api = GeneralApi::new(&client);

        let result = api.get_changelog_rss_feed().await;

        match result {
            Ok(response) => {
                println!("RSS Feed: {:?}", !response.is_empty());
                assert!(!response.is_empty(), "Expected RSS feed content");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_get_changelog_json_versions_success() {
        let client = test_client();
        let api = GeneralApi::new(&client);

        let result = api.get_changelog_json_versions().await;

        match result {
            Ok(response) => {
                println!("Changelog JSON: {:?}", response.success);
                assert!(response.success, "API responded with success=false");
                assert!(response.data.is_some(), "Expected changelog list");
                assert!(
                    !response.data.unwrap().is_empty(),
                    "Changelog list is empty"
                );
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_get_speedtest_files_success() {
        let client = test_client();
        let api = GeneralApi::new(&client);
        let result = api.get_speedtest_files(Some(FileLength::Short), None).await;

        match result {
            Ok(response) => {
                println!("Speedtest Files: {:?}", response.data);
                assert!(response.success, "API responded with success=false");
                assert!(response.data.is_some(), "Expected speedtest files list");
                assert!(
                    !response.data.unwrap().is_empty(),
                    "Speedtest file list is empty"
                );
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }
}
