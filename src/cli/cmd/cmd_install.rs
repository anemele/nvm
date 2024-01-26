use crate::core::{get_dist, get_dist_url, get_map_versions, get_path, unzip};

pub fn cmd_install(version: &str) {
    let tmp_p: Option<(std::path::PathBuf, std::path::PathBuf, std::path::PathBuf)> = get_path();
    if tmp_p.is_none() {
        return;
    }

    let (all, _, tmp) = tmp_p.unwrap();

    let (map, _) = get_map_versions();
    let map_version = match map.get(version) {
        Some(v) => v.to_string(),
        None => version.to_string(),
    };

    let dst = all.join(format!("v{}", map_version));
    if dst.exists() {
        println!("exists: {}", dst.display());
        return;
    }

    let url = get_dist_url(&map_version);
    let (_, name) = url.rsplit_once("/").unwrap();
    let zipfile = &tmp.join(name);

    if zipfile.exists() || get_dist(&url, &zipfile) {
        unzip(&zipfile, &dst);
        println!("installed: {}", map_version)
    } else {
        println!("failed to install: {}", version)
    }
}
