use crate::download::download_from_url;
use std::{path::PathBuf, process::Command};

pub fn update(base_url: &str, version: &str, temp_dir: PathBuf) -> () {
    let url: String = format!("{}rustdesk-{}-x86_64.exe", base_url, version);
    let temp_path = temp_dir.join("rustdesk.exe").display().to_string();

    download_from_url(url, &temp_path);
    Command::new("cmd")
        .arg("/C")
        .arg(&temp_path)
        .arg("--install")
        .spawn()
        .expect("Failed to install");
}
