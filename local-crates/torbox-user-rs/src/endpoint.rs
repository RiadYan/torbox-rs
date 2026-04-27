use reqwest::Method;
use torbox_core_rs::{
    client::EndpointSpec,
    data::user::{
        DeviceCodeAuth, ReferralData, SearchEngineData, SubscriptionData, TransactionData,
        UserProfile,
    },
};

use crate::{
    body::{RefreshApiTokenBody, SearchEngineControlBody, SearchEngineEditBody},
    query::{AddReferralQuery, DeviceCodeAuthQuery, SearchEngineDataQuery, UserDataQuery},
};

pub struct GetUserDataGetEp;

impl EndpointSpec for GetUserDataGetEp {
    type Req = UserDataQuery;
    type Resp = UserProfile;
    const PATH: &'static str = "api/user/me";
    const METHOD: Method = Method::GET;
}

pub struct GetConfirmationCodeGetEp;

impl EndpointSpec for GetConfirmationCodeGetEp {
    type Req = ();
    type Resp = ();
    const PATH: &'static str = "api/user/getconfirmation";
    const METHOD: Method = Method::GET;
}

pub struct RefreshApiTokenPostEp;

impl EndpointSpec for RefreshApiTokenPostEp {
    type Req = RefreshApiTokenBody;
    type Resp = String;
    const PATH: &'static str = "api/user/refreshtoken";
    const METHOD: Method = Method::POST;
}

pub struct AddReferalPostEp;

//todo: finish this and write resp struct or detail struct, will need to be changed cuz the response is absolutely hell to make since it's object inside of detail instead of data.
impl EndpointSpec for AddReferalPostEp {
    type Req = AddReferralQuery;
    type Resp = ();
    const PATH: &'static str = "api/user/addreferral";
    const METHOD: Method = Method::POST;
}

pub struct DeviceCodeAuthGetEp;

impl EndpointSpec for DeviceCodeAuthGetEp {
    type Req = DeviceCodeAuthQuery;
    type Resp = DeviceCodeAuth;
    const PATH: &'static str = "api/user/auth/device/start";
    const METHOD: Method = Method::GET;
}

pub struct ReferralDataGetEp;

impl EndpointSpec for ReferralDataGetEp {
    type Req = ();
    type Resp = ReferralData;
    const PATH: &'static str = "api/user/referraldata";
    const METHOD: Method = Method::GET;
}

pub struct SubscriptionDataGetEp;

impl EndpointSpec for SubscriptionDataGetEp {
    type Req = ();
    type Resp = SubscriptionData;
    const PATH: &'static str = "api/user/subscriptions";
    const METHOD: Method = Method::GET;
}

pub struct TransactionDataGetEp;

impl EndpointSpec for TransactionDataGetEp {
    type Req = ();
    type Resp = Vec<TransactionData>;
    const PATH: &'static str = "api/user/transactions";
    const METHOD: Method = Method::GET;
}

pub struct SearchEngineDataGetEp;

impl EndpointSpec for SearchEngineDataGetEp {
    type Req = SearchEngineDataQuery;
    type Resp = Vec<SearchEngineData>;
    const PATH: &'static str = "api/user/settings/searchengines";
    const METHOD: Method = Method::GET;
}

pub struct SearchEngineControlPostEp;

impl EndpointSpec for SearchEngineControlPostEp {
    type Req = SearchEngineControlBody;
    type Resp = ();
    const PATH: &'static str = "api/user/settings/controlsearchengines";
    const METHOD: Method = Method::GET;
}

pub struct SearchEngineEditPostEp;

impl EndpointSpec for SearchEngineEditPostEp {
    type Req = SearchEngineEditBody;
    type Resp = SearchEngineData;
    const PATH: &'static str = "api/user/settings/modifysearchengines";
    const METHOD: Method = Method::GET;
}
