use regex::bytes::Regex;
use reqwest::Client;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::error::Error;

pub async fn get_nighly_release() -> Result<Release, Box<dyn Error>> {
    let resp = Client::new()
        .get("https://api.github.com/repos/rustdesk/rustdesk/releases")
        .header("Accept", "application/json")
        .header("User-Agent", "Rust")
        .send()
        .await?;

    let releases: Vec<Release> = resp.json().await?;

    for release in releases {
        if release.name == "nightly" {
            return Ok(release);
        }
    }
    panic!("No nightly release found")
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub url: String,
    #[serde(rename = "assets_url")]
    pub assets_url: String,
    #[serde(rename = "upload_url")]
    pub upload_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub id: i64,
    pub author: Author,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    #[serde(rename = "target_commitish")]
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "published_at")]
    pub published_at: String,
    pub assets: Vec<Asset>,
    #[serde(rename = "tarball_url")]
    pub tarball_url: String,
    #[serde(rename = "zipball_url")]
    pub zipball_url: String,
    pub body: String,
    pub reactions: Option<Reactions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub url: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub uploader: Uploader,
    #[serde(rename = "content_type")]
    pub content_type: String,
    pub state: String,
    pub size: i64,
    #[serde(rename = "download_count")]
    pub download_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "browser_download_url")]
    pub browser_download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uploader {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reactions {
    pub url: String,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(rename = "+1")]
    pub n1: i64,
    #[serde(rename = "-1")]
    pub n12: i64,
    pub laugh: i64,
    pub hooray: i64,
    pub confused: i64,
    pub heart: i64,
    pub rocket: i64,
    pub eyes: i64,
}

impl Release {
    pub fn get_release_with_regex(&self, regex: &str) -> Result<String, Box<dyn Error>> {
        for asset in &self.assets {
            if Regex::new(regex)?.is_match(&asset.browser_download_url.as_bytes()) {
                return Ok(asset.browser_download_url.clone());
            }
        }
        Err("No release found".into()) // Return an error instead of panicking
    }
}

#[cfg(test)]
mod tests {
    use crate::github::get_nighly_release;
    use tokio;

    #[tokio::test]
    async fn linux_arch() {
        let release = get_nighly_release().await.unwrap();
        assert!(release.get_release_with_regex(r"^.+zst$").is_ok());
    }

    #[tokio::test]
    async fn windows() {
        let release = get_nighly_release().await.unwrap();
        assert!(release
            .get_release_with_regex(r"rustdesk-\d+.\d+.\d+-x86_64.exe")
            .is_ok());
    }

    #[tokio::test]
    async fn macos() {
        let release = get_nighly_release().await.unwrap();
        assert!(release.get_release_with_regex(r"^.+x86_64.dmg$").is_ok());
        assert!(release.get_release_with_regex(r"^.+aarch64.dmg$").is_ok())
    }

    #[tokio::test]
    async fn android() {
        let release = get_nighly_release().await.unwrap();
        assert!(release
            .get_release_with_regex("^.+armv7-signed.apk$")
            .is_ok());
        assert!(release
            .get_release_with_regex("^.+aarch64-signed.apk$")
            .is_ok())
    }
}
