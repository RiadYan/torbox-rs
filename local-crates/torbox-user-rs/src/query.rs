use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserDataQuery {
    /// Allows you to retrieve user settings.
    pub settings: bool,
}
