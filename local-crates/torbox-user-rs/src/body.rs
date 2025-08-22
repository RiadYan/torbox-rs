use serde::{Deserialize, Serialize};
use torbox_core_rs::data::user::SessionToken;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RefreshApiTokenBody {
    /// You can get this by logging into <https://torbox.app> and finding the torbox_session_token in local storage.
    pub session_token: SessionToken,
}
