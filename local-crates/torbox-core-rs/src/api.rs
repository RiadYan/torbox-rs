use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    error::{ApiError, ErrorValue},
    traits::FromBytes,
};

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<ErrorValue>,
    pub detail: Option<String>,
    pub data: Option<T>,
}

impl<T> FromBytes for ApiResponse<T>
where
    T: DeserializeOwned,
{
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, ApiError> {
        serde_json::from_slice(&bytes).map_err(ApiError::from)
    }
}
