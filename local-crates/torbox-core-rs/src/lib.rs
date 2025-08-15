pub mod api;
pub mod body;
pub mod client;
pub mod data;
pub mod error;
pub mod network;
mod tests;
pub mod traits;

//todo: divide each category (torrent, ddl, general, etc...) with feature flags to be able to disable default features and keep only necessary types.
