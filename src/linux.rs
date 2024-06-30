use crate::{download::download_from_url, github::Release};
use std::error::Error;
use std::{fs::remove_file, process::Command};

pub fn update(release: Release, tempdir: &str) -> Result<(), Box<dyn Error>> {
    let distro = std::fs::read_to_string("/etc/os-release").unwrap_or_else(|_| String::new());
    if distro.contains("arch") {
        linux_arch_download(release.get_release_with_regex(r"^.+zst$")?, tempdir);
        Ok(())
    } else {
        panic!("Unsupported distribution");
    }
}

fn linux_arch_download(url: String, temp_path: &str) {
    download_from_url(url, temp_path);
    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-U")
        .arg("--noconfirm")
        .arg(temp_path)
        .output()
        .expect("Failed to install");

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }

    remove_file(temp_path).expect("Failed to remove temp file");
}
