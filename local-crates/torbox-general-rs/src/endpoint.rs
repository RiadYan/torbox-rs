use reqwest::Method;
use torbox_core_rs::{
    client::EndpointSpec,
    data::general::{ChangelogJsonVersion, SpeedtestFile, TorboxGeneralStats},
};

use crate::query::SpeedTestQuery;

pub struct GetUpStatusEp;

impl EndpointSpec for GetUpStatusEp {
    type Req = ();
    type Resp = ();
    const PATH: &'static str = "";
    const METHOD: Method = Method::GET;
}

pub struct GetStatsEp;

impl EndpointSpec for GetStatsEp {
    type Req = ();
    type Resp = TorboxGeneralStats;
    const PATH: &'static str = "api/stats";
    const METHOD: Method = Method::GET;
}

pub struct GetChangelogRssFeedEp;

impl EndpointSpec for GetChangelogRssFeedEp {
    type Req = ();
    type Resp = String;
    const PATH: &'static str = "api/changelogs/rss";
    const METHOD: Method = Method::GET;
}

pub struct GetChangelogJsonVersionsEp;

impl EndpointSpec for GetChangelogJsonVersionsEp {
    type Req = ();
    type Resp = Vec<ChangelogJsonVersion>;
    const PATH: &'static str = "api/changelogs/json";
    const METHOD: Method = Method::GET;
}

pub struct GetSpeedtestFilesEp;

impl EndpointSpec for GetSpeedtestFilesEp {
    type Req = SpeedTestQuery;
    type Resp = Vec<SpeedtestFile>;
    const PATH: &'static str = "api/speedtest";
    const METHOD: Method = Method::GET;
}
