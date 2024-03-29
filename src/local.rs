use crate::utils::is_node_path;
use std::fs::{self, File};
use std::io;
use std::path::Path;
use zip::ZipArchive;

#[derive(Default)]
pub struct LocalVersions {
    pub current: String,
    pub versions: Vec<String>,
}

pub fn query_local(all: &Path, bin: &Path) -> Option<LocalVersions> {
    let Ok(rd) = fs::read_dir(&all) else {
        eprintln!("fail to read path: {}", all.display());
        return None;
    };

    let current = match bin.read_link() {
        Ok(link) => link.file_name().unwrap().to_str().unwrap()[1..].to_string(),
        Err(_) => String::new(),
    };

    let mut versions = vec![];
    for r in rd {
        let Ok(de) = r else {
            continue;
        };
        let dir = de.path();
        if is_node_path(&dir) {
            let name = dir.file_name().unwrap().to_str().unwrap();
            versions.push(name[1..].to_string())
        }
    }

    Some(LocalVersions { current, versions })
}

pub fn unzip(src: &Path, dst: &Path) {
    let Ok(file) = File::open(src) else {
        return;
    };

    let Ok(mut archive) = ZipArchive::new(file) else {
        return;
    };

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
            fs::create_dir_all(&outpath).unwrap();
            continue;
        }

        if let Some(p) = outpath.parent() {
            if !p.exists() {
                fs::create_dir_all(p).unwrap();
            }
        }

        let Ok(mut outfile) = File::create(&outpath) else {
            eprintln!("failed to create file: {}", outpath.display());
            continue;
        };

        io::copy(&mut fz, &mut outfile).unwrap();
    }
}
