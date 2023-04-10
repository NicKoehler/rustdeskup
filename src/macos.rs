use crate::download::download_from_url;
use std::{fs::remove_file, path::PathBuf, process::Command};

pub fn update(base_url: &str, temp_dir: PathBuf) -> () {
    let url = format!("{}rustdesk-1.2.0-x86_64.dmg", base_url);
    let temp_path = temp_dir.join("rustdesk").display().to_string();

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
