use reqwest::Method;
use torbox_core_rs::{client::EndpointSpec, data::user::UserProfile};

use crate::{body::RefreshApiTokenBody, query::UserDataQuery};

pub struct GetUserDataEp;

impl EndpointSpec for GetUserDataEp {
    type Req = UserDataQuery;
    type Resp = UserProfile;
    const PATH: &'static str = "api/user/me";
    const METHOD: Method = Method::GET;
}

pub struct GetConfirmationCodeEp;

impl EndpointSpec for GetConfirmationCodeEp {
    type Req = ();
    type Resp = ();
    const PATH: &'static str = "api/user/getconfirmation";
    const METHOD: Method = Method::GET;
}

pub struct AddReferalEp;

//todo: finish this and write resp struct or detail struct, will need to be changed cuz the response is absolutely hell to make since it's object inside of detail instead of data.
impl EndpointSpec for AddReferalEp {
    type Req = UserDataQuery;
    type Resp = ();
    const PATH: &'static str = "api/user/addreferral";
    const METHOD: Method = Method::POST;
}

pub struct RefreshApiToken;

impl EndpointSpec for RefreshApiToken {
    type Req = RefreshApiTokenBody;
    type Resp = String;
    const PATH: &'static str = "api/user/refreshtoken";
    const METHOD: Method = Method::POST;
}
