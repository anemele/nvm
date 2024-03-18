use crate::local::query_local;
use crate::semver::map_versions;
use crate::utils::get_path;
use std::fs;

pub fn exec(version: &str) {
    let Some((all, bin, _)) = get_path() else {
        return;
    };

    let Some(local_versions) = query_local(&all, &bin) else {
        return;
    };

    let (map, _) = map_versions(local_versions.versions);
    let map_version = match map.get(version) {
        Some(v) => v.to_string(),
        None => version.to_string(),
    };

    let want = all.join(&format!("v{}", map_version));
    if want.is_dir() {
        let _ = fs::remove_dir_all(want);
        println!("removed: {}", map_version);
        if map_version == local_versions.current {
            let _ = fs::remove_dir(bin);
        }
    } else {
        println!("not found: {}", version)
    }
}
