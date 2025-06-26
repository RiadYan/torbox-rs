use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

#[derive(Debug, Clone)]
pub struct ApiHeaders {
    pub headers: HeaderMap,
}

impl ApiHeaders {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).expect("Invalid token"),
        );
        Self { headers }
    }
}
