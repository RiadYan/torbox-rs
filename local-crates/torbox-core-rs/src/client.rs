use std::marker::PhantomData;

use crate::api::ApiResponse;
use crate::error::ApiError;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
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

    pub async fn call_no_body(&self, url_suffix: &str) -> Result<S::Resp, ApiError> {
        let raw: ApiResponse<S::Resp> = self.client.request(S::METHOD, url_suffix).await?;
        Self::unwrap_response(raw)
    }

    /// Full JSON body variant
    pub async fn call(&self, body: S::Req) -> Result<S::Resp, ApiError> {
        let raw: ApiResponse<S::Resp> = self
            .client
            .request_with_body(S::METHOD, S::PATH, body)
            .await?;
        Self::unwrap_response(raw)
    }

    pub async fn call_query(&self, query: S::Req) -> Result<S::Resp, ApiError>
    where
        S::Req: Serialize,
    {
        self.client
            .request_with_query(S::METHOD, S::PATH, &query)
            .await
            .map_err(ApiError::from)
    }

    fn unwrap_response(raw: ApiResponse<S::Resp>) -> Result<S::Resp, ApiError> {
        match raw {
            ApiResponse {
                success: true,
                data: Some(ok),
                ..
            } => Ok(ok),
            ApiResponse {
                success: false,
                error: Some(e),
                ..
            } => Err(ApiError::Failure(e)),
            _ => Err(ApiError::UnexpectedPayload),
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TorboxClient {
    /// Client can be specta skipped because TorboxClient should NEVER be used in any frontend, type is only used to be able to derive the APIs built from it.
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

    pub async fn request_with_query<T: DeserializeOwned, Q: Serialize>(
        &self,
        method: Method,
        endpoint: &str,
        query: &Q,
    ) -> Result<T, reqwest::Error> {
        let res = self
            .client
            .request(method, format!("{}/{}", self.base_url, endpoint))
            .headers(self.headers())
            .query(query)
            .send()
            .await?;

        res.json::<T>().await
    }
}
