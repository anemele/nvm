use super::is_node_path;
use std::fs::{self, File};
use std::io;
use std::path::Path;
use zip::ZipArchive;

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

pub fn unzip(src: &Path, dst: &Path) {
    let tmp = File::open(src);
    if tmp.is_err() {
        return;
    }

    let file = tmp.unwrap();

    let tmp = ZipArchive::new(file);
    if tmp.is_err() {
        return;
    }

    let mut archive = tmp.unwrap();
    for i in 0..archive.len() {
        let mut fz = archive.by_index(i).unwrap();
        let outpath = match fz.enclosed_name() {
            Some(p) => {
                let (_, pth) = p.to_str().unwrap().split_once("/").unwrap();
                dst.join(pth)
            }
            None => continue,
        };

        if (*fz.name()).ends_with('/') {
            // println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
            continue;
        }

        if let Some(p) = outpath.parent() {
            if !p.exists() {
                fs::create_dir_all(p).unwrap();
            }
        }
        let mut outfile = File::create(&outpath).unwrap();
        io::copy(&mut fz, &mut outfile).unwrap();
    }
}
