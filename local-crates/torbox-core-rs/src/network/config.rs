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

#[cfg(feature = "ureq")]
#[derive(Debug, Clone)]
pub struct ApiHeadersUreq {
    pub headers: Vec<(String, String)>,
}

#[cfg(feature = "ureq")]
impl ApiHeadersUreq {
    pub fn new(token: &str) -> Self {
        Self {
            headers: vec![
                ("Content-Type".into(), "application/json".into()),
                ("Accept".into(), "application/json".into()),
                ("Authorization".into(), format!("Bearer {}", token)),
            ],
        }
    }
}
