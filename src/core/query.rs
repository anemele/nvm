use super::{get_path, is_node_path};
use std::fs;

pub fn query_local() -> Option<(String, Vec<String>)> {
    let tmp = get_path();
    if tmp.is_none() {
        eprintln!("fail to get paths");
        return None;
    }

    let (all, bin, _) = tmp.unwrap();
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
