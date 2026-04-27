use reqwest::Method;
use torbox_core_rs::{
    client::EndpointSpec,
    data::{
        creation::DownloadLinkResponse,
        webdownload::{
            WebDownloadCacheAvailability, WebdownloadCreationResponse, WebdownloadHosterList,
            WebdownloadStatus,
        },
    },
    enums::OneOrMany,
};

use crate::{
    body::{WebdownloadControlReq, WebdownloadCreateBody},
    query::{
        ListWebdownloadsQuery, WebdownloadCachedAvailabilityQuery, WebdownloadRequestLinkQuery,
    },
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
    const METHOD: Method = Method::GET;
}

pub struct ListWebdownloadsGetEp;

impl EndpointSpec for ListWebdownloadsGetEp {
    type Req = ListWebdownloadsQuery;
    type Resp = Option<OneOrMany<WebdownloadStatus>>;
    const PATH: &'static str = "api/webdl/mylist";
    const METHOD: Method = Method::GET;
}

/// Takes in a list of comma separated usenet hashes and checks if the web download is cached.
///
/// This endpoint only gets a max of around 100 at a time, due to http limits in queries. If you want to do more, you can simply do more hash queries
pub struct WebdownloadCachedAvailabilityGetEp;

impl EndpointSpec for WebdownloadCachedAvailabilityGetEp {
    type Req = WebdownloadCachedAvailabilityQuery;
    type Resp = Option<OneOrMany<WebDownloadCacheAvailability>>;
    const PATH: &'static str = "api/webdl/checkcached";
    const METHOD: Method = Method::GET;
}

pub struct WebdownloadHosterListGetEp;

impl EndpointSpec for WebdownloadHosterListGetEp {
    type Req = ();
    type Resp = Vec<WebdownloadHosterList>;
    const PATH: &'static str = "api/webdl/hosters";
    const METHOD: Method = Method::GET;
}
