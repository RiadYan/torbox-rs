// use reqwest::Method;

// use crate::{api::ApiResponse, data::user::UserProfile, tests::load_token_from_file};

// #[tokio::test]
// async fn test_user_profile_endpoint_settings() {
//     let token = load_token_from_file().expect("Missing TORBOX_TOKEN in .token file");
//     let client = TorboxClientLegacy::new(token);

//     let response: ApiResponse<UserProfile> = client
//         .request(Method::GET, "api/user/me?settings=true")
//         .await
//         .expect("Failed to fetch user profile with settings");

//     assert!(response.success, "API responded with success=false");
//     assert!(response.data.is_some(), "Expected user profile data");

//     println!("Successfully gotten user profile with settings");
// }

// #[tokio::test]
// async fn test_user_profile_endpoint_no_settings() {
//     let token = load_token_from_file().expect("Missing TORBOX_TOKEN in .token file");
//     let client = TorboxClientLegacy::new(token);

//     let response: ApiResponse<UserProfile> = client
//         .request(Method::GET, "api/user/me?settings=false")
//         .await
//         .expect("Failed to fetch user profile without settings");

//     assert!(response.success, "API responded with success=false");
//     assert!(response.data.is_some(), "Expected user profile data");

//     println!("Successfully gotten user profile without settings");
// }
