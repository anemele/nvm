use crate::core::get_path;
use std::fs;

pub fn cmd_list() {
    if let Some((all, _bin, _)) = get_path() {
        println!("all available:");
        if let Ok(rd) = fs::read_dir(all) {
            for de in rd {
                if let Ok(dir) = de {
                    println!("  {}", dir.file_name().to_str().unwrap())
                }
            }
        }
        // println!("* {}", bin.file_name().unwrap().to_str().unwrap())
    } else {
        println!("no available.")
    };
}
