use crate::{api::ErrorValue, data::ApiDataResponse};

// #[cfg_attr(feature = "specta", derive(specta::Type))]

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("API returned success = false: {0:?}")]
    Failure(ErrorValue),

    #[error("Unexpected ApiDataResponse variant: {0:?}")]
    Unexpected(ApiDataResponse),

    #[error("Unexpected Payload variant")]
    UnexpectedPayload,
}
