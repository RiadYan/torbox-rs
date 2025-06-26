use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::{Client, Method};
use serde::{Serialize, de::DeserializeOwned};
use specta::Type;

#[derive(Clone, Type)]
pub struct TorboxClient {
    #[specta(skip)]
    pub client: Client,
    pub token: String,
    pub base_url: String,
}

impl TorboxClient {
    pub fn new(token: String) -> Self {
        let client = Client::new();
        Self {
            client,
            token,
            base_url: "https://api.torbox.app/v1".to_string(),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap(),
        );
        headers
    }

    pub async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
    ) -> Result<T, reqwest::Error> {
        let res = self
            .client
            .request(method, format!("{}/{}", self.base_url, endpoint))
            .headers(self.headers())
            .send()
            .await?;

        res.json::<T>().await
    }

    pub async fn request_with_body<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        endpoint: &str,
        body: B,
    ) -> Result<T, reqwest::Error> {
        let res = self
            .client
            .request(method, format!("{}/{}", self.base_url, endpoint))
            .headers(self.headers())
            .json(&body)
            .send()
            .await?;

        res.json::<T>().await
    }
}
