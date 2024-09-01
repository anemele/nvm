use crate::local::query_local;
use crate::semver::map_versions;
use crate::utils::get_path;
use std::fs;

pub fn exec(version: &str) -> anyhow::Result<()> {
    let (all, bin, _) = get_path()?;

    let local_versions = query_local(&all)?;

    let (map, _) = map_versions(local_versions.versions);
    let map_version = match map.get(version) {
        Some(v) => v.to_string(),
        None => version.to_string(),
    };

    let want = all.join(&format!("v{}", map_version));
    if want.is_dir() {
        if fs::remove_dir_all(want).is_ok() {
            println!("removed: {}", map_version);
            if map_version == local_versions.current {
                let _ = fs::remove_dir(bin);
            }
        } else {
            eprintln!("failed to remove: {}", map_version)
        }
    } else {
        println!("not found: {}", version)
    }

    Ok(())
}
