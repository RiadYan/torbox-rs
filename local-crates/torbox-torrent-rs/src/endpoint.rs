use reqwest::Method;
use torbox_core_rs::{api::ApiResponse, client::EndpointSpec, data::torrent::TorrentStatus};

use crate::{
    body::{TorrentControlBody, TorrentCreateBody, TorrentInfoBody},
    payload::{TorrentCreatePayload, TorrentInfoPayload},
    query::{ListTorrentsQuery, TorrentInfoQuery, TorrentStatusQuery},
};

pub struct TorrentCreatePostEp;

impl EndpointSpec for TorrentCreatePostEp {
    type Req = TorrentCreateBody;
    type Resp = TorrentCreatePayload;
    const PATH: &'static str = "api/torrents/createtorrent";
    const METHOD: Method = Method::POST;
}

pub struct ListTorrentsGetEp;

impl EndpointSpec for ListTorrentsGetEp {
    type Req = ListTorrentsQuery;
    type Resp = Option<Vec<TorrentStatus>>;
    const PATH: &'static str = "api/torrents/mylist";
    const METHOD: Method = Method::GET;
}

pub struct TorrentStatusGetEp;

impl EndpointSpec for TorrentStatusGetEp {
    type Req = TorrentStatusQuery;
    type Resp = Option<TorrentStatus>;
    const PATH: &'static str = "api/torrents/mylist";
    const METHOD: Method = Method::GET;
}

pub struct TorrentInfoPostEp;

impl EndpointSpec for TorrentInfoPostEp {
    type Req = TorrentInfoBody;
    type Resp = TorrentInfoPayload;
    const PATH: &'static str = "api/torrents/torrentinfo";
    const METHOD: Method = Method::POST;
}

pub struct TorrentInfoGetEp;

impl EndpointSpec for TorrentInfoGetEp {
    type Req = TorrentInfoQuery;
    type Resp = TorrentInfoPayload;
    const PATH: &'static str = "api/torrents/torrentinfo";
    const METHOD: Method = Method::GET;
}

pub struct TorrentControlPostEp;

impl EndpointSpec for TorrentControlPostEp {
    type Req = TorrentControlBody;
    type Resp = ApiResponse<()>;
    const PATH: &'static str = "api/torrents/controltorrent";
    const METHOD: Method = Method::POST;
}
