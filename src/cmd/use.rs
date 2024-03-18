use crate::local::query_local;
use crate::semver::map_versions;
use crate::utils::get_path;
use std::process::Command;
use std::{fs, process::Stdio};

pub fn exec(version: &str) {
    let Some((all, bin, _)) = get_path() else {
        return;
    };

    let Some(local_versions) = query_local(&all, &bin) else {
        return;
    };

    let (map, _) = map_versions(local_versions.versions);

    let map_version = match map.get(version) {
        Some(s) => s.to_string(),
        None => version.to_string(),
    };

    if map_version == local_versions.current {
        println!("current version is in use: {}", map_version);
        return;
    }

    let want = all.join(&format!("v{}", map_version));
    if !want.exists() {
        println!("not found: {}", version);
        return;
    }

    if bin.exists() {
        let _ = fs::remove_dir(&bin);
    }
    match Command::new("cmd.exe")
        .args([
            "/c",
            "mklink",
            "/j",
            bin.to_str().unwrap(),
            want.to_str().unwrap(),
        ])
        .stdout(Stdio::null())
        .status()
    {
        Ok(code) => {
            if code.code().unwrap() == 0 {
                println!("use {}", map_version)
            } else {
                println!("fail to use {}", version)
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
