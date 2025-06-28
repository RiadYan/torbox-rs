use serde::{Deserialize, Serialize};

use crate::data::ApiDataResponse;

// #[cfg_attr(feature = "specta", derive(specta::Type))]

/// Represents all possible TorBox error codes returned from the API.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ApiErrorCode {
    /// Could not access internal database/memory store information
    DatabaseError,

    /// The reason for the error is unknown. Usually there will be error data attached in the "data" key.
    ///
    /// In these cases please report the request to `contact@torbox.app`.
    UnknownError,

    /// There are no provided credentials.
    NoAuth,

    /// The provided token is invalid.
    BadToken,

    /// There was an error verifying the given authentication.
    AuthError,

    /// The provided option is invalid.
    InvalidOption,

    /// The server tried redirecting, but it faulted.
    RedirectError,

    /// The server tried verifying your OAuth token, but it was not accepted by the provider.
    OauthVerificationError,

    /// You have hit an endpoint that doesn't exist.
    EndpointNotFound,

    /// The item you queried cannot be found.
    ItemNotFound,

    /// This feature is restricted to users of higher plans.
    ///
    /// The user is recommended to upgrade their plan to use this endpoint.
    PlanRestrictedFeature,

    /// This item already exists.
    DuplicateItem,

    /// The RSS feed is invalid or not a well-formed XML.
    BozoRssFeed,

    /// There was an error with the Sellix API. Usually in the case of payments.
    SellixError,

    /// Client sent too much data to the API.
    ///
    /// Please keep requests under 100MB in size.
    TooMuchData,

    /// This download is oversized for the user's plan.
    ///
    /// The user is recommended to upgrade their plan to download this file.  
    ///
    /// - **Free** Plan Limit: `10737418240` bytes  
    /// - **Essential** Plan Limit: `214748364800` bytes  
    /// - **Standard** Plan Limit: `214748364800` bytes  
    /// - **Pro** Plan Limit: `536870912000` bytes
    DownloadTooLarge,

    /// The API is missing required information to process the request.
    MissingRequiredOption,

    /// Client sent too many options.
    ///
    /// Usually this has to do with the API requiring only 1 option but the client sent more than the required.
    TooManyOptions,

    /// The torrent sent is not a valid torrent.
    BozoTorrent,

    /// There are no download servers available to handle this request.
    ///
    /// This should never happen. If you receieve this error, please contact us at `contact@torbox.app`.
    NoServersAvailableError,

    /// User has hit the maximum monthly limit.
    ///
    /// It is recommended user upgrade their account to be able to download more.
    MonthlyLimit,

    /// User is on download cooldown.
    ///
    /// It is recommended user upgrade their account to be able to bypass this restriction.
    CooldownLimit,

    /// User has hit their max active download limit.
    ///
    /// It is recommended user upgrade their account or purchase addons to bypass this restriction.
    ActiveLimit,

    /// There was an error interacting with the download on the download server.
    ///
    /// It is recommdned to simply wait some time before trying again.
    DownloadServerError,

    /// The NZB sent is not a valid NZB file.
    BozoNzb,

    /// There was an error searching using the TorBox Search API.
    SearchError,

    /// The client is sending requests from the incorrect device.
    InvalidDevice,

    /// The request parameters sent does not allow for this request to complete.
    DiffIssue,

    /// The link given is inaccessible or has no online files.
    LinkOffline,

    /// This vendor account has been disabled.
    ///
    /// Please contact support.
    VendorDisabled,

    /// The regex entered is invalid.
    BozoRegex,

    /// The confirmation code is invalid.
    BadConfirmation,

    /// The confirmation code has expired.
    ///
    /// Request a new code.
    ConfirmationExpired,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("JSON Error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("API returned success = false: {0:?}")]
    Failure(ErrorValue),

    #[error("Unexpected ApiDataResponse variant: {0:?}")]
    Unexpected(ApiDataResponse),

    #[error("Unexpected Payload variant")]
    UnexpectedPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(untagged)]
pub enum ErrorValue {
    Code(ApiErrorCode),
    Bool(bool),
    Message(String),
}
