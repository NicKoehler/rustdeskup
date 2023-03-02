use crate::download::download_from_url;
use std::{fs::remove_file, path::PathBuf, process::Command};

const FILENAME: &str = "rustdesk";

pub fn update(base_url: &str, temp_dir: PathBuf) -> () {
    let distro = std::fs::read_to_string("/etc/os-release").unwrap_or_else(|_| String::new());

    if distro.contains("arch") {
        linux_arch_download(
            format!("{}rustdesk-1.2.0-0-x86_64.pkg.tar.zst", base_url),
            &temp_dir.join(FILENAME).display().to_string(),
        );
    } else {
        panic!("Unsupported distribution");
    }
}

fn linux_arch_download(url: String, temp_path: &str) {
    download_from_url(url, temp_path);
    Command::new("sudo")
        .arg("pacman")
        .arg("-U")
        .arg(temp_path)
        .arg("--noconfirm")
        .output()
        .expect("Failed to install");
    remove_file(temp_path).unwrap();
}
