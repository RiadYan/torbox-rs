use crate::{
    query::WebdownloadControlQuery,
    types::{WebdownloadControlSource, WebdownloadOperation},
};
use async_trait::async_trait;
use reqwest::multipart::Form;
use serde::Serialize;
use torbox_core_rs::body::ToMultipart;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub struct WebdownloadCreateBody {
    pub link: String,
    pub password: Option<String>,
    pub name: Option<String>,
    /// Tells TorBox you want this web download instantly queued. Optional.
    pub as_queued: Option<bool>,
    /// Only adds the download if it is cached on TorBox. If not cached, it won't be added.
    pub add_only_if_cached: Option<bool>,
}

#[async_trait]
impl ToMultipart for WebdownloadCreateBody {
    async fn to_multipart(self) -> Form {
        let mut __form__ = Form::new();
        __form__ = __form__.text("link", self.link);
        if let Some(name) = self.name {
            __form__ = __form__.text("name", name);
        }
        if let Some(password) = self.password {
            __form__ = __form__.text("password", password);
        }
        if let Some(queued) = self.as_queued {
            __form__ = __form__.text("as_queued", queued.to_string());
        }
        if let Some(add_only_if_cached) = self.add_only_if_cached {
            __form__ = __form__.text("add_only_if_cached", add_only_if_cached.to_string());
        }
        __form__
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadControlReq {
    #[serde(flatten)]
    // the web download's id, optional if using the "all" parameter.
    pub source: WebdownloadControlSource,
    // the operation you want to perform on the web download
    pub operation: WebdownloadOperation,
    pub bypass_cache: bool,
}

impl Default for WebdownloadControlReq {
    fn default() -> Self {
        Self {
            source: WebdownloadControlSource::All(false),
            operation: WebdownloadOperation::Delete,
            bypass_cache: false,
        }
    }
}

impl WebdownloadControlReq {
    pub fn into_parts(self) -> (WebdownloadControlQuery, WebdownloadControlBody) {
        (
            WebdownloadControlQuery {
                bypass_cache: self.bypass_cache,
            },
            WebdownloadControlBody {
                source: self.source,
                operation: self.operation,
            },
        )
    }
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadControlBody {
    #[serde(flatten)]
    pub source: WebdownloadControlSource,
    pub operation: WebdownloadOperation,
}
