# torbox-rs 🦀

[![crates.io](https://img.shields.io/crates/v/torbox-rs)](https://crates.io/crates/torbox-rs)
[![docs.rs](https://docs.rs/torbox-rs/badge.svg)](https://docs.rs/torbox-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

The official Rust SDK for TorBox's torrent services. Query, manage, and download torrents with full rust experience.

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
torbox-rs = "0.0.1-alpha.2"
```

Need TypeScript type generation? Add the `specta` feature:

```toml
[dependencies.torbox-rs]
version = "0.0.1-alpha.2"
features = ["specta"]  # Enables Specta support for Tauri/JS interop
```

## 🚀 Quick Start

Here's how to get rolling with the basics:

```rust
use torbox_rs::{TorboxClient, TorrentApi};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client with your API token
    let client = TorboxClient::new("your_api_token_here".into());
    let api = TorrentApi::new(&client);

    // List all your torrents
    let torrents = api.list_torrents_query(Default::default()).await?;
    println!("My Torrents: {:#?}", torrents.data);

    // Create new torrent from magnet link
    let new_torrent = api.create_torrent(TorrentCreateBody {
        source: "magnet:?xt=urn:btih:EXAMPLE_HASH".into(),
        seed: None,
        allow_zip: true,
        name: Some("ubuntu-22.04".into()),
        as_queued: None,
    }).await?;

    // Export as .torrent file
    let torrent_file = api.export_data_query(TorrentExportDataQuery {
        torrent_id: new_torrent.data.unwrap().id,
        data_type: TorrentExportType::File,
    }).await?;

    std::fs::write("ubuntu.torrent", torrent_file)?;

    // Get download link (always use redirect: true for permalinks!)
    let download_link = api.request_download_link(TorrentRequestLinkQuery {
        token: api.token().to_string(),
        torrent_id: new_torrent.data.unwrap().id,
        file_id: None,
        zip_link: true,
        user_ip: None,
        redirect: true,  // Set this to true for direct download links it'll help you and the devs !
    }).await?;

    println!("Download ready at: {}", download_link);
    Ok(())
}
```

## 🔍 Key Features

### 📥 Torrent Management
```rust
// Pause/resume/delete torrents
api.control_torrent(TorrentControlBody {
    source: TorrentId(123),  // Use torrent ID or hash
    operation: TorrentOperation::Pause,  // Pause, Resume, Delete
}).await?;
```

### 🔄 Dual Response Handling
The API smartly handles both formats:
```rust
match api.export_data_query(query).await? {
    TorrentExportResponse::Json(json) => { /* Get magnet URI */ },
    TorrentExportResponse::File(data) => { /* Save .torrent file */ }
}
```

### 📊 Torrent Inspection
```rust
// Get detailed info (with 5s timeout)
let info = api.info_query(TorrentInfoQuery {
    hash: "6cad4a4671eb622b279619868342171cf5ec1045".into(),
    timeout: Some(5),
}).await?;
```

## 💡 Pro Tips
1. **Permalinks**: Always use `redirect: true` when generating download links
2. **Caching**: Torrent lists update every 10 minutes unless forced
3. **Errors**: Check `ApiError` variants for proper error handling

## 📚 Documentation
- [Full API Docs](https://docs.rs/torbox-rs)
- Local docs: `cargo doc --open`

## 🤝 Need Help?
Open an issue if you have any issues! I will be happy to help.
