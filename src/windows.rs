use crate::{download::download_from_url, github::Release};
use std::error::Error;
use std::process::Command;

pub fn update(release: Release, tempdir: &str) -> Result<(), Box<dyn Error>> {
    download_from_url(
        release.get_release_with_regex(r"rustdesk-\d+.\d+.\d+-x86_64.exe")?,
        &format!("{tempdir}.exe"),
    );
    Command::new("cmd")
        .arg("/C")
        .arg(tempdir)
        .arg("--silent-install")
        .spawn()
        .expect("Failed to install");
    Ok(())
}
