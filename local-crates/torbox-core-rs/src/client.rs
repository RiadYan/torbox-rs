use std::any::TypeId;
use std::marker::PhantomData;

use crate::api::ApiResponse;
use crate::body::ToMultipart;
use crate::error::ApiError;
use crate::traits::FromBytes;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::multipart::Form;
use reqwest::{Client, Method};
use serde::{Serialize, de::DeserializeOwned};

pub trait EndpointSpec {
    /// JSON body you send - Use `()` to not send anything.
    type Req: serde::Serialize;
    /// Typed payload you expect back on success
    type Resp: serde::de::DeserializeOwned;

    const PATH: &'static str;
    const METHOD: Method = Method::POST;
}

pub struct Endpoint<'c, S: EndpointSpec> {
    client: &'c TorboxClient,
    _marker: PhantomData<S>,
}

impl<'c, S: EndpointSpec> Endpoint<'c, S> {
    pub fn new(client: &'c TorboxClient) -> Self {
        Self {
            client,
            _marker: PhantomData,
        }
    }

    pub async fn call_no_body(&self, url_suffix: &str) -> Result<ApiResponse<S::Resp>, ApiError>
    where
        S::Resp:,
        <S as EndpointSpec>::Resp: std::fmt::Debug,
    {
        self.client.request(S::METHOD, url_suffix).await
    }

    pub async fn call(&self, body: S::Req) -> Result<ApiResponse<S::Resp>, ApiError> {
        self.client
            .request_with_json(S::METHOD, S::PATH, body)
            .await
    }

    pub async fn call_query(&self, query: S::Req) -> Result<ApiResponse<S::Resp>, ApiError>
    where
        S::Req: Serialize,
    {
        self.client
            .request_with_query(S::METHOD, S::PATH, &query)
            .await
    }

    pub async fn call_multipart(&self, body: S::Req) -> Result<ApiResponse<S::Resp>, ApiError>
    where
        S::Req: ToMultipart + Send + Sync,
    {
        let form = body.to_multipart().await;
        self.client
            .request_multipart(S::METHOD, S::PATH, form)
            .await
    }

    pub async fn call_query_raw(&self, query: S::Req) -> Result<Vec<u8>, ApiError>
    where
        S::Req: Serialize,
    {
        let url = format!("{}/{}", self.client.base_url, S::PATH);
        let response = self
            .client
            .client
            .request(S::METHOD, &url)
            .headers(self.client.headers("application/json"))
            .query(&query)
            .send()
            .await?;

        Ok(response.bytes().await?.to_vec())
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorboxClient {
    /// Client can be specta skipped because TorboxClient should NEVER be used in any frontend, type is only used to be able to derive the APIs built from it.
    #[specta(skip)]
    pub client: Client,
    pub(crate) token: String,
    pub base_url: String,
}

impl TorboxClient {
    pub fn new(token: String) -> Self {
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        Self {
            client,
            token,
            base_url: "https://api.torbox.app/v1".to_string(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    async fn parse_response<T>(&self, res: reqwest::Response) -> Result<T, ApiError>
    where
        T: DeserializeOwned + FromBytes,
    {
        let content_type = res
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if content_type.starts_with("application/json") {
            let text = res.text().await?;
            serde_json::from_str::<T>(&text).map_err(ApiError::from)
        } else {
            // Handle binary responses
            let bytes = res.bytes().await?.to_vec();
            T::from_bytes(bytes)
        }
    }

    fn headers(&self, _content_type: &'static str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap(),
        );
        headers
    }

    pub async fn request_multipart<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
        form: Form,
    ) -> Result<T, ApiError> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let res = self
            .client
            .request(method, url)
            .headers(self.headers("multipart/form-data"))
            .multipart(form)
            .send()
            .await?;

        let text = res.text().await?;

        let parsed = serde_json::from_str::<T>(&text)?;
        Ok(parsed)
    }

    pub async fn request<T: DeserializeOwned + FromBytes>(
        &self,
        method: Method,
        endpoint: &str,
    ) -> Result<T, ApiError> {
        let res = self
            .client
            .request(method, format!("{}/{}", self.base_url, endpoint))
            .headers(self.headers("application/json"))
            .send()
            .await?;

        self.parse_response::<T>(res).await
    }

    pub async fn request_with_json<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        endpoint: &str,
        body: B,
    ) -> Result<T, ApiError> {
        let res = self
            .client
            .request(method, format!("{}/{}", self.base_url, endpoint))
            .headers(self.headers("application/json"))
            .json(&body)
            .send()
            .await?;

        let text = res.text().await?;

        let parsed = serde_json::from_str::<T>(&text)?;
        Ok(parsed)
    }

    pub async fn request_with_query<T: DeserializeOwned, Q: Serialize>(
        &self,
        method: Method,
        endpoint: &str,
        query: &Q,
    ) -> Result<T, ApiError> {
        let res = self
            .client
            .request(method, format!("{}/{}", self.base_url, endpoint))
            .headers(self.headers("application/json"))
            .query(query)
            .send()
            .await?;

        let text = res.text().await?;
        // eprintln!("Raw API response: {}", text);

        let parsed = serde_json::from_str::<T>(&text)?;
        Ok(parsed)
    }
}
