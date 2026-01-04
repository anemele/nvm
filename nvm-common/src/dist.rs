use std::env::consts::{ARCH, OS};

pub struct Dist {
    pub dir: String,
    pub ext: String,
}

pub fn get_dist(version: &str) -> Dist {
    // node-v{v}-{os}-{arch}.{ext}
    // v:   {semver}
    // os:  win, linux, darwin
    // arc: x64, x86, arm64, ...
    // ext: zip, 7z, tar.gz, tar.xz

    let os = match OS {
        "windows" => "win",
        "macos" => "darwin",
        x => x,
    };
    let arch = match ARCH {
        "x86_64" => "x64",
        "arm" => "arm64",
        x => x,
    };
    let dir = format!("node-v{version}-{os}-{arch}");
    // use .7z on Windows, tar.xz on *NIX for a smaller size.
    let ext = match os {
        "win" => "7z",
        _ => "tar.xz",
    }
    .to_string();
    Dist { dir, ext }
}
