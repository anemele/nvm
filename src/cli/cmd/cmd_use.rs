use crate::core::get_path;
use std::process::Command;
use std::{fs, process::Stdio};

pub fn cmd_use(version: &str) {
    if let Some((all, bin, _)) = get_path() {
        let want = all.join(&format!("v{}", version));
        if !want.exists() {
            println!("not found: {}", version);
            return;
        }

        if bin.exists() {
            let _ = fs::remove_dir(&bin);
        }
        let _ = Command::new("cmd")
            .args([
                "/c",
                "mklink",
                "/j",
                bin.to_str().unwrap(),
                want.to_str().unwrap(),
            ])
            .stdout(Stdio::null())
            .status();
    }
}
