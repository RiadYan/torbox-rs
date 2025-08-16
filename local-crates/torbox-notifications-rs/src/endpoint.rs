use reqwest::Method;
use torbox_core_rs::{client::EndpointSpec, data::user::UserProfile};

pub struct GetRssNotifFeedEp;

impl EndpointSpec for GetRssNotifFeedEp {
    type Req = String;
    type Resp = String;
    const PATH: &'static str = "api/notifications/rss";
    const METHOD: Method = Method::GET;
}

pub struct GetNotifFeedEp;

impl EndpointSpec for GetNotifFeedEp {
    type Req = String;
    type Resp = ();
    const PATH: &'static str = "api/notifications/mynotifications";
    const METHOD: Method = Method::GET;
}

pub struct ClearAllNotificationsEp;

impl EndpointSpec for ClearAllNotificationsEp {
    type Req = ();
    type Resp = ();
    const PATH: &'static str = "api/notifications/clear";
    const METHOD: Method = Method::POST;
}

pub struct ClearSingleNotificationEp;

impl EndpointSpec for ClearSingleNotificationEp {
    type Req = String;
    type Resp = ();
    const PATH: &'static str = "api/notifications/clear";
    const METHOD: Method = Method::POST;
}

pub struct SendTestNotificationEp;

impl EndpointSpec for SendTestNotificationEp {
    type Req = ();
    type Resp = ();
    const PATH: &'static str = "api/notifications/test";
    const METHOD: Method = Method::POST;
}
