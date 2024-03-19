use std::fs::remove_dir_all;

use crate::utils::get_path;

pub fn exec() {
    let Some((_, _, tmp)) = get_path() else {
        return;
    };

    if remove_dir_all(&tmp).is_err() {
        eprintln!("failed to clean cache, do it manually. ({})", tmp.display());
    }
}
