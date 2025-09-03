use reqwest::Method;
use torbox_core_rs::{
    client::EndpointSpec,
    data::{
        creation::DownloadLinkResponse,
        webdownload::{WebdownloadCreationResponse, WebdownloadStatus},
    },
    enums::OneOrMany,
};

use crate::{
    body::{WebdownloadControlReq, WebdownloadCreateBody},
    query::{ListWebdownloadsQuery, WebdownloadRequestLinkQuery},
};

pub struct WebdownloadCreatePostEp;

impl EndpointSpec for WebdownloadCreatePostEp {
    type Req = WebdownloadCreateBody;
    type Resp = WebdownloadCreationResponse;
    const PATH: &'static str = "api/webdl/createwebdownload";
    const METHOD: Method = Method::POST;
}

pub struct WebdownloadControlPostEp;

impl EndpointSpec for WebdownloadControlPostEp {
    type Req = WebdownloadControlReq;
    type Resp = ();
    const PATH: &'static str = "api/webdl/controlwebdownload";
    const METHOD: Method = Method::POST;
}

pub struct WebdownloadRequestLinkGetEp;

impl EndpointSpec for WebdownloadRequestLinkGetEp {
    type Req = WebdownloadRequestLinkQuery;
    type Resp = DownloadLinkResponse;
    const PATH: &'static str = "api/webdl/requestdl";
    const METHOD: Method = Method::POST;
}

pub struct ListWebdownloadsGetEp;

impl EndpointSpec for ListWebdownloadsGetEp {
    type Req = ListWebdownloadsQuery;
    type Resp = Option<OneOrMany<WebdownloadStatus>>;
    const PATH: &'static str = "api/webdl/mylist";
    const METHOD: Method = Method::POST;
}
