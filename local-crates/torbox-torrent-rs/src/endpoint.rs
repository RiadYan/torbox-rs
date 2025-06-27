use reqwest::Method;
use torbox_core_rs::{api::ApiResponse, client::EndpointSpec, data::torrent::TorrentStatus};

use crate::{
    body::{TorrentCreateBody, TorrentInfoBody},
    payload::{TorrentCreatePayload, TorrentInfoPayload},
    query::{ListTorrentsQuery, TorrentInfoQuery},
};

pub struct TorrentCreateEp;

impl EndpointSpec for TorrentCreateEp {
    type Req = TorrentCreateBody;
    type Resp = ApiResponse<TorrentCreatePayload>;
    const PATH: &'static str = "api/torrent/create";
    const METHOD: Method = Method::POST;
}

pub struct ListTorrentsGetEp;

impl EndpointSpec for ListTorrentsGetEp {
    type Req = ListTorrentsQuery; // used as query params
    type Resp = ApiResponse<Option<Vec<TorrentStatus>>>;
    const PATH: &'static str = "api/torrents/mylist";
    const METHOD: Method = Method::GET;
}

pub struct TorrentInfoPostEp;

impl EndpointSpec for TorrentInfoPostEp {
    type Req = TorrentInfoBody;
    type Resp = ApiResponse<TorrentInfoPayload>;
    const PATH: &'static str = "api/torrents/torrentinfo";
    const METHOD: Method = Method::POST;
}

/* GET endpoint marker  */
pub struct TorrentInfoGetEp;

impl EndpointSpec for TorrentInfoGetEp {
    /// GET uses no JSON body, so `()` (unit) is fine.
    type Req = TorrentInfoQuery;
    type Resp = ApiResponse<TorrentInfoPayload>;
    const PATH: &'static str = "api/torrents/torrentinfo";
    const METHOD: Method = Method::GET;
}
