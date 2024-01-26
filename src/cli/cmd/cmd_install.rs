use crate::core::{get_dist, get_dist_url, get_path, unzip};

pub fn cmd_install(version: &str) {
    if let Some((all, _, tmp)) = get_path() {
        let dst = all.join(format!("v{}", version));
        if dst.exists() {
            println!("exists: {}", dst.display());
            return;
        }
        let url = get_dist_url(version);
        let (_, name) = url.rsplit_once("/").unwrap();
        let zipfile = &tmp.join(name);
        if get_dist(&url, &zipfile) {
            unzip(&zipfile, &dst);
        }
    }
}
