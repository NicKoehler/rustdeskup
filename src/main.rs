mod download;
mod linux;
mod windows;

use std::env;

const BASE_URL: &str = "https://github.com/rustdesk/rustdesk/releases/download/nightly/";

fn main() {
    let os = std::env::consts::OS;
    let temp_dir = env::temp_dir();

    println!("Downloading the update");

    match os {
        "linux" => linux::update(BASE_URL, temp_dir),
        "windows" => windows::update(BASE_URL, temp_dir),
        _ => panic!("Unupported OS."),
    };

    println!("RustDesk has been updated successfully!");
}
