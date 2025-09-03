use async_trait::async_trait;
use reqwest::multipart::Form;
use serde::Serialize;
use torbox_core_rs::body::ToMultipart;

use crate::types::{WebdownloadControlSource, WebdownloadOperation};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadCreateBody {
    pub link: String,
    pub password: Option<String>,
    pub name: Option<String>,
    /// Tells TorBox you want this web download.instantly queued. Optional.
    pub as_queued: Option<bool>,
    /// Only adds the download if it is cached on TorBox. If not cached, it won't be added.
    pub add_only_if_cached: Option<bool>,
}

#[async_trait]
impl ToMultipart for WebdownloadCreateBody {
    async fn to_multipart(self) -> Form {
        let mut form = Form::new();

        form = form.text("link", self.link);

        if let Some(name) = self.name {
            form = form.text("name", name);
        }

        if let Some(password) = self.password {
            form = form.text("password", password);
        }

        if let Some(queued) = self.as_queued {
            form = form.text("as_queued", queued.to_string());
        }

        form
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadControlBody {
    #[serde(flatten)]
    // the web download's id, optional if using the "all" parameter.
    pub source: WebdownloadControlSource,
    // the operation you want to perform on the torrent
    pub operation: WebdownloadOperation,
}
