use std::env;

use dotenvy::from_filename;

mod torrent;
mod user;
mod webdownload;

pub fn load_token_from_file() -> Option<String> {
    if let Err(err) = from_filename(".token") {
        eprintln!("Could not load .token file: {err}");
        return None;
    }
    env::var("TORBOX_TOKEN").ok()
}
