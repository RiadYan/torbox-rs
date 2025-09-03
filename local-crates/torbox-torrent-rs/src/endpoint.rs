use reqwest::Method;
use torbox_core_rs::{client::EndpointSpec, data::torrent::TorrentStatus, enums::OneOrMany};

use crate::{
    body::{TorrentControlBody, TorrentCreateBody, TorrentInfoBody},
    payload::{TorrentCreatePayload, TorrentInfoPayload},
    query::{
        ListTorrentsQuery, TorrentExportDataQuery, TorrentInfoQuery, TorrentRequestLinkQuery,
        TorrentStatusQuery,
    },
    types::{TorrentDownloadResponse, TorrentExportResponse, TorrentSource},
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
    type Resp = Option<OneOrMany<TorrentStatus>>;
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
    type Resp = ();
    const PATH: &'static str = "api/torrents/controltorrent";
    const METHOD: Method = Method::POST;
}

pub struct TorrentRequestLinkGetEp;

impl EndpointSpec for TorrentRequestLinkGetEp {
    type Req = TorrentRequestLinkQuery;
    type Resp = TorrentDownloadResponse;
    const PATH: &'static str = "api/torrents/requestdl";
    const METHOD: Method = Method::GET;
}

pub struct TorrentExportDataGetEp;

impl EndpointSpec for TorrentExportDataGetEp {
    type Req = TorrentExportDataQuery;
    type Resp = String; // to String since we're getting magnet links in JSON maybe idk
    const PATH: &'static str = "api/torrents/exportdata";
    const METHOD: Method = Method::GET;
}
