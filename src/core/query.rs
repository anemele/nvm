use super::is_node_path;
use std::{fs, path::Path};

pub fn query_local(all: &Path, bin: &Path) -> Option<(String, Vec<String>)> {
    let tmp = fs::read_dir(&all);
    if tmp.is_err() {
        eprintln!("fail to read path: {}", all.display());
        return None;
    }

    let current = if let Ok(link) = bin.read_link() {
        link.file_name().unwrap().to_str().unwrap()[1..].to_string()
    } else {
        String::new()
    };

    let mut versions = vec![];
    for de in tmp.unwrap() {
        if de.is_err() {
            continue;
        }
        let dir = de.unwrap().path();
        if is_node_path(&dir) {
            let name = dir.file_name().unwrap().to_str().unwrap();
            versions.push(name[1..].to_string())
        }
    }

    Some((current, versions))
}
