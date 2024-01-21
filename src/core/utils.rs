use super::{get_node_home, NODE_ALL, NODE_BIN, NODE_TMP};
use super::{NODE_BASE_URL, NPM_BASE_URL};
use serde_json::Value;
use std::fs::File;
use std::io;
use std::path::Path;
use std::{fs, path::PathBuf};
use zip::ZipArchive;

pub fn is_lts(lts: Value) -> bool {
    match lts {
        Value::String(_) => true,
        Value::Bool(b) => b,
        _ => false,
    }
}

pub fn get_node_url(path: &str) -> String {
    NODE_BASE_URL.to_string() + path
}

pub fn get_npm_url(path: &str) -> String {
    NPM_BASE_URL.to_string() + path
}

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

pub fn unzip(src: &Path, dst: &Path) {
    if let Ok(file) = File::open(src) {
        if let Ok(mut archive) = ZipArchive::new(file) {
            for i in 0..archive.len() {
                let mut fz = archive.by_index(i).unwrap();
                let outpath = match fz.enclosed_name() {
                    Some(p) => {
                        let (_, pth) = p.to_str().unwrap().split_once("/").unwrap();
                        dst.join(pth)
                    }
                    None => continue,
                };
                // {
                //     let comment = fz.comment();
                //     if !comment.is_empty() {
                //         println!("File {i} comment: {comment}");
                //     }
                // }

                if (*fz.name()).ends_with('/') {
                    // println!("File {} extracted to \"{}\"", i, outpath.display());
                    fs::create_dir_all(&outpath).unwrap();
                } else {
                    // println!(
                    //     "File {} extracted to \"{}\" ({} bytes)",
                    //     i,
                    //     outpath.display(),
                    //     fz.size()
                    // );
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            fs::create_dir_all(p).unwrap();
                        }
                    }
                    let mut outfile = fs::File::create(&outpath).unwrap();
                    io::copy(&mut fz, &mut outfile).unwrap();
                }
            }
        }
    }
}
