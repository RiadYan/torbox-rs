use reqwest::Method;
use torbox_core_rs::{api::ApiResponse, client::EndpointSpec, data::torrent::TorrentStatus};

use crate::{
    body::{TorrentCreateBody, TorrentInfoBody},
    payload::{TorrentCreatePayload, TorrentInfoPayload},
    query::{ListTorrentsQuery, TorrentInfoQuery, TorrentStatusQuery},
};

pub struct TorrentCreatePostEp;

impl EndpointSpec for TorrentCreatePostEp {
    type Req = TorrentCreateBody;
    type Resp = ApiResponse<TorrentCreatePayload>;
    const PATH: &'static str = "api/torrent/create";
    const METHOD: Method = Method::POST;
}

pub struct ListTorrentsGetEp;

impl EndpointSpec for ListTorrentsGetEp {
    type Req = ListTorrentsQuery;
    type Resp = ApiResponse<Option<Vec<TorrentStatus>>>;
    const PATH: &'static str = "api/torrents/mylist";
    const METHOD: Method = Method::GET;
}

pub struct TorrentStatusGetEp;

impl EndpointSpec for TorrentStatusGetEp {
    type Req = TorrentStatusQuery;
    type Resp = ApiResponse<Option<TorrentStatus>>;
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

pub struct TorrentInfoGetEp;

impl EndpointSpec for TorrentInfoGetEp {
    type Req = TorrentInfoQuery;
    type Resp = ApiResponse<TorrentInfoPayload>;
    const PATH: &'static str = "api/torrents/torrentinfo";
    const METHOD: Method = Method::GET;
}
