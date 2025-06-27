use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(untagged)]
pub enum ErrorValue {
    Bool(bool),
    Message(String),
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<ErrorValue>,
    pub detail: Option<String>,
    pub data: Option<T>,
}
