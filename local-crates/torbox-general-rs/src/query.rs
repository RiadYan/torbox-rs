use serde::{Deserialize, Serialize};

use crate::types::FileLength;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SpeedTestQuery {
    /// Determines the size of the file used for the speedtest. May be "long" or "short". Optional.
    pub test_length: Option<FileLength>,
    /// Determines what cdns are returned.
    /// May be any region that TorBox is located in. To get this value, send a request without this value to determine all of the available regions that are available.
    pub region: Option<String>,
}
