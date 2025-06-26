use crate::{network::config::ApiHeaders, structs::ApiResponse, tests::load_token_from_file};

#[tokio::test]
async fn test_user_profile_endpoint_settings() {
    let token = load_token_from_file().expect("Missing TORBOX_TOKEN in .token file");
    let headers = ApiHeaders::new(&token);

    let client = reqwest::Client::new();
    let res = client
        .get("https://api.torbox.app/v1/api/user/me?settings=true")
        .headers(headers.headers.clone())
        .send()
        .await
        .expect("Request failed");

    assert!(
        res.status().is_success(),
        "API returned error status: {}",
        res.status()
    );

    let response: ApiResponse = res
        .json()
        .await
        .expect("Failed to deserialize into ApiResponse");

    response
        .data
        .expect("Expected user profile data in API response");

    println!("Successfully gotten user profile !");
}
