#[cfg(target_family = "windows")]
pub const FILENAME: &str = "rustdesk.exe";

#[cfg(target_family = "unix")]
pub const FILENAME: &str = "rustdesk";
