use crate::core::get_path;
use std::fs;

pub fn cmd_uninstall(version: &str) {
    if let Some((all, _, _)) = get_path() {
        let want = all.join(&version);
        if want.exists() {
            let _ = fs::remove_dir_all(want);
            return;
        }
    }

    println!("not found: {}", version)
}
