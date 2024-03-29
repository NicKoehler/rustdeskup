use std::process::Command;

pub fn update_rustdeskup() {
    println!("Checking rustdeskup updates");
    Command::new("cargo")
        .arg("install")
        .arg("--git")
        .arg("https://github.com/nickoehler/rustdeskup");
}
