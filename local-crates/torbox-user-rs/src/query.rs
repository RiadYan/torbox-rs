use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UserDataQuery {
    /// Allows you to retrieve user settings.
    pub settings: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DeviceCodeAuthQuery {
    /// The app name that the user will see when verifying the code. This should be the name of your app.
    pub app: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AddReferralQuery {
    /// A referral code. Must be UUID.
    pub referral: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TransactionPdfQuery {
    pub transaction_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SearchEngineDataQuery {
    /// Retrieve a specific search engine if it is owned by the user and exists.
    pub id: u32,
}
