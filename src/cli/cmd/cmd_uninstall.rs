use crate::core::{get_path, map_versions, query_local};
use std::fs;

pub fn cmd_uninstall(version: &str) {
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
    let map_version = match map.get(version) {
        Some(v) => v.to_string(),
        None => version.to_string(),
    };

    let want = all.join(&format!("v{}", map_version));
    if want.is_dir() {
        let _ = fs::remove_dir_all(want);
        println!("removed: {}", map_version);
        if map_version == current {
            let _ = fs::remove_dir(bin);
        }
    } else {
        println!("not found: {}", version)
    }
}
