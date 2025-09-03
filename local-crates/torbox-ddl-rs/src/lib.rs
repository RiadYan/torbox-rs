use torbox_core_rs::client::TorboxClient;

//todo: Add the rest for ddl
pub mod body;
pub mod endpoint;
pub mod payload;
pub mod query;
pub mod tests;
pub mod types;

#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct WebdownloadApi<'a> {
    client: &'a TorboxClient,
}

impl<'a> WebdownloadApi<'a> {
    pub fn new(client: &'a TorboxClient) -> Self {
        Self { client }
    }
}
