use serde::{Deserialize, Serialize};

use crate::data::ApiDataResponse;

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(untagged)]
pub enum ErrorValue {
    Bool(bool),
    Message(String),
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiResponse {
    pub success: bool,
    pub error: Option<ErrorValue>,
    pub detail: Option<String>,
    pub data: Option<ApiDataResponse>,
}
