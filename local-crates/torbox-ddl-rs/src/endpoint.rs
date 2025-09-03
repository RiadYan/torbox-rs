use reqwest::Method;
use torbox_core_rs::{client::EndpointSpec, data::webdownload::WebdownloadCreationResponse};

use crate::body::{WebdownloadControlBody, WebdownloadCreateBody};

pub struct WebdownloadCreatePostEp;

impl EndpointSpec for WebdownloadCreatePostEp {
    type Req = WebdownloadCreateBody;
    type Resp = WebdownloadCreationResponse;
    const PATH: &'static str = "api/webdl/createwebdownload";
    const METHOD: Method = Method::POST;
}

pub struct WebdownloadControlPostEp;

impl EndpointSpec for WebdownloadControlPostEp {
    type Req = WebdownloadControlBody;
    type Resp = ();
    const PATH: &'static str = "api/webdl/controlwebdownload";
    const METHOD: Method = Method::POST;
}
