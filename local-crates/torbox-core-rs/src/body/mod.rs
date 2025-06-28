use async_trait::async_trait;
use reqwest::multipart::Form;

#[async_trait]
pub trait ToMultipart {
    async fn to_multipart(self) -> Form;
}
