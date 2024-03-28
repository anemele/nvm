use crate::utils::get_path;
use std::fs;

pub fn exec() {
    let Some((_, _, tmp)) = get_path() else {
        return;
    };

    if fs::remove_dir_all(&tmp).is_err() {
        eprintln!("failed to clean cache, do it manually. ({})", tmp.display());
    }
}
