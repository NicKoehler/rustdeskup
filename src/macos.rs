use crate::download::download_from_url;
use std::{fs::remove_file, path::PathBuf, process::Command};

pub fn update(base_url: &str, version: &str, temp_dir: PathBuf) -> () {
    let url = format!("{}rustdesk-{}-x86_64.dmg", base_url, version);
    let temp_path = temp_dir.join("rustdesk").display().to_string();

    download_from_url(url, &temp_path);
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sudo hdiutil attach {};sudo cp -R /Volumes/rustdesk-{}/RustDesk.app /Applications;sudo hdiutil unmount /Volumes/rustdesk-{}",
            temp_path, version, version
        ))
        .output()
        .expect("Failed to install");
    remove_file(temp_path).expect("Failed to remove temp file");
}
