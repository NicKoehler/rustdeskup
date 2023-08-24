mod android;
mod download;
mod linux;
mod macos;
mod windows;

use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BASE_URL: &str = "https://github.com/rustdesk/rustdesk/releases/download/nightly/";

fn main() {
    let os = std::env::consts::OS;
    let temp_dir = env::temp_dir();

    println!("Downloading the update");

    match os {
        "linux" => linux::update(BASE_URL, VERSION, temp_dir),
        "macos" => macos::update(BASE_URL, VERSION, temp_dir),
        "android" => android::update(BASE_URL, VERSION, temp_dir),
        "windows" => windows::update(BASE_URL, VERSION, temp_dir),
        _ => panic!("Unupported OS."),
    };

    println!("RustDesk has been updated successfully!");
}
