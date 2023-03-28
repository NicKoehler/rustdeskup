use crate::download::download_from_url;
use std::{env::consts::ARCH, fs::remove_file, path::PathBuf, process::Command};

const FILENAME: &str = "rustdesk";

pub fn update(base_url: &str, temp_dir: PathBuf) -> () {
    let url = match ARCH {
        "x86_64" => format!("{}rustdesk-1.2.0-x86_64-apple-darwin.dmg", base_url),
        "aarch64" => format!("{}rustdesk-1.2.0-aarch64-apple-darwin.dmg", base_url),
        _ => panic!("Unsupported processor"),
    };

    let temp_path = temp_dir.join(FILENAME).display().to_string();

    download_from_url(url, &temp_path);
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sudo hdiutil attach {};sudo cp -R /Volumes/rustdesk-1.2.0/RustDesk.app /Applications;sudo hdiutil unmount /Volumes/rustdesk-1.2.0",
            temp_path
        ))
        .output()
        .expect("Failed to install");
    remove_file(temp_path).unwrap();
}
