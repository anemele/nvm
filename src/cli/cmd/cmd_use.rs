use crate::core::{get_path, map_versions, query_local};
use std::process::Command;
use std::{fs, process::Stdio};

pub fn cmd_use(version: &str) {
    let tmp = get_path();
    if tmp.is_none() {
        return;
    }

    let (all, bin, _) = tmp.unwrap();

    let tmp = query_local(&all, &bin);
    if tmp.is_none() {
        return;
    }

    let (current, versions) = tmp.unwrap();
    let (map, _) = map_versions(versions);

    let o: String;
    let map_version = match map.get(version) {
        Some(s) => {
            o = s.to_string();
            o.as_str()
        }
        None => version,
    };

    if map_version == current {
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
