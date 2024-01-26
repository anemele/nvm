use super::{get_node_home, NODE_ALL, NODE_BIN, NODE_TMP};
use std::{fs, path::PathBuf};

pub fn get_path() -> Option<(PathBuf, PathBuf, PathBuf)> {
    let home = get_node_home();
    if !home.exists() {
        if let Err(e) = fs::create_dir(&home) {
            eprintln!("{}", e);
            return None;
        }
    }

    let all = home.join(NODE_ALL);
    if !all.exists() {
        if let Err(e) = fs::create_dir(&all) {
            eprintln!("{}", e);
            return None;
        }
    }

    let tmp = home.join(NODE_TMP);
    if !tmp.exists() {
        if let Err(e) = fs::create_dir(&tmp) {
            eprintln!("{}", e);
            return None;
        }
    }

    let bin = home.join(NODE_BIN);

    Some((all, bin, tmp))
}
