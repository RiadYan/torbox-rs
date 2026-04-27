use serde::{Deserialize, Serialize};
use torbox_core_rs::data::user::SessionToken;

use crate::types::{
    SearchEngineControlOperation, SearchEngineControlSource, SearchEngineEditSource,
};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RefreshApiTokenBody {
    /// You can get this by logging into <https://torbox.app> and finding the torbox_session_token in local storage.
    pub session_token: SessionToken,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SearchEngineControlBody {
    #[serde(flatten)]
    pub source: SearchEngineControlSource,
    pub operation: SearchEngineControlOperation,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SearchEngineEditBody {
    pub id: u64,
    #[serde(flatten)]
    pub source: SearchEngineEditSource,
    pub apikey: Option<String>,
    pub download_type: String,
}
