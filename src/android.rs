use crate::{download::download_from_url, github::Release};
use std::{env::consts::ARCH, error::Error, fs::remove_file, process::Command};

pub fn update(release: Release, tempdir: &str) -> Result<(), Box<dyn Error>> {
    let url = match ARCH {
        "arm" => release.get_release_with_regex("^.+armv7-signed.apk$")?,
        "aarch64" => release.get_release_with_regex("^.+aarch64-signed.apk$")?,
        _ => panic!("Unsupported processor"),
    };

    download_from_url(url, tempdir);
    Command::new("sh")
        .arg("-c")
        .arg(format!("termux-open {}", tempdir))
        .output()
        .expect("Failed to install");
    remove_file(tempdir).expect("Failed to remove temp file");
    Ok(())
}
