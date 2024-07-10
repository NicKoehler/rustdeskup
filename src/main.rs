mod android;
mod download;
mod github;
mod linux;
mod macos;
mod windows;

use github::get_nighly_release;
use std::env::consts::OS;
use tempdir::TempDir;

#[tokio::main]
async fn main() {
    println!("Fetching nightly releases");
    let release = get_nighly_release()
        .await
        .expect("Nightly release not found.");
    let tempdir = &TempDir::new("rustdesk")
        .unwrap()
        .into_path()
        .join("rustdesk")
        .display()
        .to_string();

    match OS {
        "linux" => linux::update(release, tempdir).await,
        "macos" => macos::update(release, tempdir).await,
        "windows" => windows::update(release, tempdir).await,
        "android" => android::update(release, tempdir).await,
        _ => panic!("Unupported OS."),
    }
    .expect("Failed to update rustdesk");

    println!("RustDesk has been updated successfully!");
}
