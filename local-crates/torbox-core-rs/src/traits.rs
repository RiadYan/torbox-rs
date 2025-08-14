use crate::error::ApiError;
use serde::de::DeserializeOwned;

/// Convert raw bytes into a type
pub trait FromBytes: Sized {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, ApiError>;
}

// Raw bytes passthrough
impl FromBytes for Vec<u8> {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, ApiError> {
        Ok(bytes)
    }
}

// UTF-8 string
impl FromBytes for String {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, ApiError> {
        String::from_utf8(bytes).map_err(|e| ApiError::Custom(e.to_string()))
    }
}

// Helper for JSON types
pub fn from_bytes_json<T: DeserializeOwned>(bytes: Vec<u8>) -> Result<T, ApiError> {
    serde_json::from_slice(&bytes).map_err(ApiError::from)
}
