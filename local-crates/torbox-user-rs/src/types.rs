use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum SearchEngineControlOperation {
    Enable,
    Disable,
    Check,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum SearchEngineControlSource {
    Id(u32),
    All(bool),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum SearchEngineEditSource {
    Type(String),
    Url(String),
}
