use serde::{Deserialize, Serialize};

use crate::error::ErrorValue;

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<ErrorValue>,
    pub detail: Option<String>,
    pub data: Option<T>,
}
