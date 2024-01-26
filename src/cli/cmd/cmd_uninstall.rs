use crate::core::get_path;
use std::fs;

pub fn cmd_uninstall(version: &str) {
    let tmp = get_path();
    if tmp.is_none() {
        return;
    }

    let (all, _, _) = tmp.unwrap();
    let want = all.join(&version);
    if want.exists() {
        let _ = fs::remove_dir_all(want);
    } else {
        println!("not found: {}", version)
    }
}
