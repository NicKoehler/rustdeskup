use crate::{download::download_from_url, github::Release};
use regex::Regex;
use std::error::Error;
use std::{fs::remove_file, process::Command};

pub fn update(release: Release, tempdir: &str) -> Result<(), Box<dyn Error>> {
    #[cfg(target_arch = "x86_64")]
    let url = release.get_release_with_regex(r"^.+x86_64.dmg$")?;
    #[cfg(target_arch = "aarch64")]
    let url = release.get_release_with_regex(r"^.+aarch64.dmg$")?;

    let version = extract_version(&url)?;

    download_from_url(url, tempdir);
    Command::new("sh")
        .arg("-c")
        .arg(
            format!(
            "sudo hdiutil attach {tempdir};sudo cp -R /Volumes/rustdesk-{version}/RustDesk.app /Applications;sudo hdiutil unmount /Volumes/rustdesk-{version}",
        ))
        .output()
        .expect("Failed to install");
    remove_file(tempdir).expect("Failed to remove temp file");
    Ok(())
}

pub fn extract_version(url: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"\-(\d+.\d+.\d+)\-").unwrap();
    if let Some(captures) = re.captures(url) {
        return Ok(captures.get(1).map(|v| v.as_str()).unwrap().to_string());
    }
    panic!("Cannot parse version")
}

#[cfg(test)]
mod tests {
    use crate::macos::extract_version;

    #[test]
    fn version_test() {
        let version = "https://github.com/rustdesk/rustdesk/releases/download/nightly/rustdesk-1.2.7-aarch64.dmg";
        assert!(extract_version(version).unwrap() == "1.2.7")
    }
}
