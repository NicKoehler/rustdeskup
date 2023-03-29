use crate::consts::FILENAME;
use crate::download::download_from_url;
use std::{env::consts::ARCH, fs::remove_file, path::PathBuf, process::Command};

pub fn update(base_url: &str, temp_dir: PathBuf) -> () {
    let url = match ARCH {
        "arm" => format!(
            "{}rustdesk-1.2.0-armv7-linux-androideabi-release-signed.apk",
            base_url
        ),
        "aarch64" => format!(
            "{}rustdesk-1.2.0-aarch64-linux-android-release-signed.apk",
            base_url
        ),
        _ => panic!("Unsupported processor"),
    };

    let temp_path = temp_dir.join(FILENAME).display().to_string();

    download_from_url(url, &temp_path);
    Command::new("sh")
        .arg("-c")
        .arg(format!("termux-open {}", temp_path))
        .output()
        .expect("Failed to install");
    remove_file(temp_path).unwrap();
}
