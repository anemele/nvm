use crate::remote::{download_dist, get_map_versions};
use crate::utils::get_path;
use std::env::consts::{ARCH, OS};
use std::fs;

struct Dist {
    dir: String,
    url: String,
}

fn get_dist(version: &str) -> Option<Dist> {
    // node-v{v}-{os}-{arch}.{ext}
    // v:   {semver}
    // os:  win, linux, darwin
    // arc: x64, x86, arm64, ...
    // ext: zip, 7z, tar.gz, tar.xz

    // use .7z on Windows, .xz on *NIX for a smaller size.
    let (os, ext) = match OS {
        "linux" => ("linux", "tar.xz"),
        "macos" => ("darwin", "tar.xz"),
        "windows" => ("win", "7z"),
        _ => {
            eprintln!("unsupported OS: {OS}");
            return None;
        }
    };
    let arch = match ARCH {
        "x86" => "x86",
        "x86_64" => "x64",
        "arm" => "arm64",
        _ => {
            eprintln!("unsupported ARCH: {ARCH}");
            return None;
        }
    };
    let dir = format!("node-v{version}-{os}-{arch}");
    let url = format!("https://nodejs.org/dist/v{version}/{dir}.{ext}");
    Some(Dist { dir, url })
}

pub fn exec(version: &str) {
    let Some((all, _, tmp)) = get_path() else {
        return;
    };

    let Some((map, _)) = get_map_versions() else {
        return;
    };

    let map_version = match map.get(version) {
        Some(v) => v.to_string(),
        None => version.to_string(),
    };

    let dst = all.join(format!("v{}", map_version));
    if dst.exists() {
        println!("exists: {}", dst.display());
        return;
    }

    let Some(dist) = get_dist(&map_version) else {
        eprintln!("no dist found");
        return;
    };

    let (_, name) = dist.url.rsplit_once("/").unwrap();
    let src = tmp.join(name);

    if !src.exists() && !download_dist(&dist.url, &src) {
        eprintln!("failed to download: {}", version);
        return;
    }

    #[cfg(target_family = "unix")]
    let ok = {
        use std::process::Command;
        Command::new("tar")
            .arg("-xf")
            .arg(src)
            .arg("-C")
            .arg(&all)
            .status()
            .is_ok_and(|s| s.success())
    };

    #[cfg(target_family = "windows")]
    let ok = { sevenz_rust::decompress_file(src, &all).is_ok() };

    if ok && fs::rename(all.join(dist.dir), dst).is_ok() {
        println!("installed: {}", map_version)
    } else {
        println!("failed to install: {}", version)
    }
}
