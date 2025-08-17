#[cfg(test)]
pub mod notification_test {
    use std::env;

    use crate::NotificationApi;
    use dotenvy::from_filename;
    use torbox_core_rs::{
        client::TorboxClient, data::notifications::NotificationFeed, error::ApiError,
    };

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
    async fn test_get_rss_notif_feed_success() {
        let client = test_client();
        let api = NotificationApi::new(&client);

        let result = api.get_rss_notif_feed().await;

        match result {
            Ok(feed) => {
                println!("RSS Feed Length: {}", feed.len());
                assert!(!feed.is_empty(), "RSS feed should not be empty");
                assert!(feed.contains("<rss"), "Should contain RSS markup");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_get_notif_feed_success() {
        let client = test_client();
        let api = NotificationApi::new(&client);

        let result = api.get_notif_feed().await;

        match result {
            Ok(response) => {
                println!("Notification Feed: {:?}", response);
                assert!(response.success, "API responded with success=false");
                assert!(response.data.is_some(), "Expected notification feed");

                let feed = response.data.unwrap();
                if !feed.is_empty() {
                    println!("Latest notification: {:?}", feed[0]);
                }
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_clear_single_notification() {
        let client = test_client();
        let api = NotificationApi::new(&client);

        // First get notifications to find one to clear
        let notifs = api.get_notif_feed().await.unwrap();
        if let Some(notif_list) = notifs.data {
            if !notif_list.is_empty() {
                let test_id = notif_list[0].id;
                let result = api.clear_notification(test_id).await;

                match result {
                    Ok(response) => {
                        println!("Clear Notification Response: {:?}", response);
                        assert!(response.success, "API responded with success=false");
                    }
                    Err(e) => panic!("API call failed: {e:?}"),
                }
            } else {
                println!("No notifications available to test clearing");
            }
        } else {
            println!("No notification data received");
        }
    }

    #[tokio::test]
    async fn test_clear_all_notifications() {
        let client = test_client();
        let api = NotificationApi::new(&client);

        let result = api.clear_all_notifications().await;

        match result {
            Ok(response) => {
                println!("Clear All Response: {:?}", response);
                assert!(response.success, "API responded with success=false");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_send_test_notification() {
        let client = test_client();
        let api = NotificationApi::new(&client);

        let result = api.send_test_notification().await;

        match result {
            Ok(response) => {
                println!("Test Notification Response: {:?}", response);
                assert!(response.success, "API responded with success=false");

                // Verify the test notification appears in feed
                let notifs = api.get_notif_feed().await.unwrap();
                if let Some(notif_list) = notifs.data {
                    assert!(
                        notif_list.iter().any(|n| n.title.contains("Test")),
                        "Test notification not found in feed"
                    );
                }
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }
}
