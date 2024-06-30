mod android;
mod download;
mod github;
mod linux;
mod macos;
mod windows;
use std::env::consts::OS;

use github::get_nighly_release;
use tempdir::TempDir;

fn main() {
    let release = get_nighly_release().expect("Nightly release not found.");
    let tempdir = &TempDir::new("rustdesk")
        .unwrap()
        .into_path()
        .join("rustdesk")
        .display()
        .to_string();

    match OS {
        "linux" => linux::update(release, tempdir),
        "macos" => macos::update(release, tempdir),
        "windows" => windows::update(release, tempdir),
        "android" => android::update(release, tempdir),
        _ => panic!("Unupported OS."),
    }
    .expect("Failed to update rustdesk");

    println!("RustDesk has been updated successfully!");
}
