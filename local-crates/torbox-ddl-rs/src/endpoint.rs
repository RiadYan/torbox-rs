use reqwest::Method;
use torbox_core_rs::{client::EndpointSpec, data::webdownload::WebDownloadCreationResponse};

use crate::body::{WebDownloadControlBody, WebDownloadCreateBody};

pub struct WebDownloadCreatePostEp;

impl EndpointSpec for WebDownloadCreatePostEp {
    type Req = WebDownloadCreateBody;
    type Resp = WebDownloadCreationResponse;
    const PATH: &'static str = "api/webdl/createwebdownload";
    const METHOD: Method = Method::POST;
}

pub struct WebDownloadControlPostEp;

impl EndpointSpec for WebDownloadControlPostEp {
    type Req = WebDownloadControlBody;
    type Resp = WebDownloadCreationResponse;
    const PATH: &'static str = "api/webdl/controlwebdownload";
    const METHOD: Method = Method::POST;
}
