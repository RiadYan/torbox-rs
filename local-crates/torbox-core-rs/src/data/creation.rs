use serde::{Deserialize, Serialize};

use crate::api::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub enum DownloadLinkResponse {
    Json(ApiResponse<String>),
    Redirect(String),
}
