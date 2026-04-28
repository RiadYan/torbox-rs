#[cfg(test)]
pub mod webdownload_test {
    use crate::{
        WebdownloadApi,
        body::{WebdownloadControlReq, WebdownloadCreateBody},
        query::{
            ListWebdownloadsQuery, WebdownloadCachedAvailabilityQuery, WebdownloadRequestLinkQuery,
        },
        types::WebdownloadControlSource,
    };
    use dotenvy::from_filename;
    use std::env;
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
    async fn test_list_webdownloads_success() {
        let client = test_client();
        let api = WebdownloadApi::new(&client);
        let query = ListWebdownloadsQuery::default();
        let result = api.list_query(query).await;
        match result {
            Ok(response) => {
                println!("Webdownload List: {:?}", response);
                assert!(response.success, "API responded with success=false");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_list_hosters_success() {
        let client = test_client();
        let api = WebdownloadApi::new(&client);
        let result = api.list_hosters().await;
        match result {
            Ok(response) => {
                assert!(response.success, "API responded with success=false");
                assert!(response.data.is_some(), "Expected at least one hoster");
                println!("Hoster List: {:?}", response.data.unwrap().len());
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_create_webdownload_success() {
        let client = test_client();
        let api = WebdownloadApi::new(&client);
        let body = WebdownloadCreateBody {
            link: "https://example.com/testfile.zip".to_string(),
            ..Default::default()
        };
        let result = api.create(body).await;
        match result {
            Ok(response) => {
                println!("Create Webdownload: {:?}", response);
                assert!(response.success, "API responded with success=false");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_control_webdownload_success() {
        let client = test_client();
        let api = WebdownloadApi::new(&client);

        let body = WebdownloadCreateBody {
            link: "https://example.com/testfile.zip".to_string(),
            ..Default::default()
        };
        let created = api
            .create(body)
            .await
            .expect("Failed to create webdownload");
        assert!(created.success, "Create responded with success=false");
        let web_id = created
            .data
            .expect("No data in create response")
            .webdownload_id;

        let req = WebdownloadControlReq {
            source: WebdownloadControlSource::WebdlId(web_id),
            ..Default::default()
        };
        let result = api.control(req).await;
        match result {
            Ok(response) => {
                println!("Control Webdownload: {:?}", response);
                assert!(response.success, "API responded with success=false");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_request_download_link_json_success() {
        let client = test_client();
        let api = WebdownloadApi::new(&client);
        let query = WebdownloadRequestLinkQuery {
            token: load_token_from_file().expect("Missing TORBOX_TOKEN in .token file"),
            web_id: 1,
            redirect: false,
            ..Default::default()
        };
        let result = api.request_download_link(query).await;
        match result {
            Ok(response) => {
                println!("Download Link (JSON): {:?}", response);
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_request_download_link_redirect_success() {
        let client = test_client();
        let api = WebdownloadApi::new(&client);
        let query = WebdownloadRequestLinkQuery {
            token: load_token_from_file().expect("Missing TORBOX_TOKEN in .token file"),
            web_id: 1,
            redirect: true,
            ..Default::default()
        };
        let result = api.request_download_link(query).await;
        match result {
            Ok(response) => {
                println!("Download Link (Redirect): {:?}", response);
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_is_cached_success() {
        let client = test_client();
        let api = WebdownloadApi::new(&client);
        let query = WebdownloadCachedAvailabilityQuery::default();
        let result = api.is_cached(query).await;
        match result {
            Ok(response) => {
                println!("Cache Availability: {:?}", response);
                assert!(response.success, "API responded with success=false");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }
}
