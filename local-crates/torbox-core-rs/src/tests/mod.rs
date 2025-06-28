use std::env;

use dotenvy::from_filename;

mod user;
mod webdownload;

pub fn load_token_from_file() -> Option<String> {
    if let Err(err) = from_filename(".token") {
        eprintln!(
            "Could not load .token file, please create one before starting unit testing: {err}"
        );
        return None;
    }
    env::var("TORBOX_TOKEN").ok()
}
