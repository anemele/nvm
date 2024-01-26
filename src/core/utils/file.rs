use std::fs::{self, File};
use std::io;
use std::path::Path;
use zip::ZipArchive;

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
        // {
        //     let comment = fz.comment();
        //     if !comment.is_empty() {
        //         println!("File {i} comment: {comment}");
        //     }
        // }

        if (*fz.name()).ends_with('/') {
            // println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
            continue;
        }
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
        let mut outfile = File::create(&outpath).unwrap();
        io::copy(&mut fz, &mut outfile).unwrap();
    }
}
