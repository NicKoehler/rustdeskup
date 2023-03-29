use crate::consts::FILENAME;
use crate::download::download_from_url;
use std::{path::PathBuf, process::Command};

pub fn update(base_url: &str, temp_dir: PathBuf) -> () {
    let url: String = format!("{}rustdesk-1.2.0-x86_64-pc-windows-msvc.exe", base_url);
    let temp_path = temp_dir.join(FILENAME).display().to_string();

    download_from_url(url, &temp_path);
    Command::new("cmd")
        .arg("/C")
        .arg(&temp_path)
        .arg("--reinstall")
        .spawn()
        .expect("Failed to install");
}
