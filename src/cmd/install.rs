use crate::local::unzip;
use crate::remote::{get_dist, get_map_versions};
use crate::utils::{get_dist_url, get_path};

pub fn exec(version: &str) {
    let Some((all, _, tmp)) = get_path() else {
        return;
    };

    let Some((map, _)) = get_map_versions() else {
        return;
    };

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
