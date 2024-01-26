use crate::core::{get_path, is_node_path};
use std::fs;

pub fn cmd_list() {
    let tmp = get_path();
    if tmp.is_none() {
        return;
    }

    let (all, bin, _) = tmp.unwrap();
    let tmp = fs::read_dir(all);
    if tmp.is_err() {
        return;
    }

    let current = if let Ok(link) = bin.read_link() {
        link.file_name().unwrap().to_str().unwrap().to_string()
    } else {
        String::new()
    };

    let mut count = 0;
    for de in tmp.unwrap() {
        if de.is_err() {
            continue;
        }
        let dir = de.unwrap().path();
        if is_node_path(&dir) {
            let name = dir.file_name().unwrap().to_str().unwrap();
            if name == current {
                println!("\x1b[32m* {}\x1b[m", name)
            } else {
                println!("  {}", name)
            }
            count += 1;
        }
    }
    if count == 0 {
        println!("no available. install first.")
    }
}
