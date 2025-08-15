#[cfg(test)]
pub mod user_test {
    use std::env;

    use crate::{UserApi, query::UserDataQuery};
    use dotenvy::from_filename;
    use torbox_core_rs::{client::TorboxClient, data::user::UserProfile};

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
    async fn test_get_user_data_success() {
        let client = test_client();
        let api = UserApi::new(&client);

        let result = api.get_user_data(false).await;

        match result {
            Ok(response) => {
                println!("User Data: {:?}", response);
                assert!(response.success, "API responded with success=false");
                assert!(response.data.is_some(), "Expected user profile data");

                let profile = response.data.unwrap();
                assert!(profile.id != 0, "User ID should not be empty");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }

        let result_with_settings = api.get_user_data(true).await;

        match result_with_settings {
            Ok(response) => {
                println!("User Data with Settings: {:?}", response);
                assert!(response.success);
                if let Some(profile) = response.data {
                    println!("User settings: {:?}", profile.settings);
                }
            }
            Err(e) => panic!("API call failed with settings: {e:?}"),
        }
    }

    #[tokio::test]
    async fn test_get_confirmation_code_success() {
        let client = test_client();
        let api = UserApi::new(&client);

        let result = api.get_confirmation_code().await;

        match result {
            Ok(response) => {
                println!("Confirmation Code Response: {:?}", response);
                assert!(response.success, "API responded with success=false");
            }
            Err(e) => panic!("API call failed: {e:?}"),
        }
    }

    /// Not to be tested, too problematic
    async fn _test_refresh_api_token_success() {}
}
