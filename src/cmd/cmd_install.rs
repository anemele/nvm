use indicatif::ProgressBar;
use std::env::consts::{ARCH, OS};
use std::{fs, time};

struct Dist {
    dir: String,
    ext: String,
}

fn get_dist(version: &str) -> Dist {
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
    // use .7z on Windows, tar.xz on *NIX for a smaller size.
    let ext = match os {
        "win" => "7z",
        _ => "tar.xz",
    };
    let arch = match ARCH {
        "x86_64" => "x64",
        "arm" => "arm64",
        x => x,
    };
    let dir = format!("node-v{version}-{os}-{arch}");
    Dist {
        dir,
        ext: ext.to_string(),
    }
}

pub fn exec(version: &str) -> anyhow::Result<()> {
    let paths = crate::utils::get_paths()?;
    let (map, _) = crate::remote::get_map_versions()?;

    let map_version = match map.get(version) {
        Some(v) => v.to_string(),
        None => version.to_string(),
    };

    let dst = paths.all.join(format!("v{}", map_version));
    if dst.exists() {
        println!("exists: {}", dst.display());
        return Ok(());
    }

    let dist = get_dist(&map_version);

    let file = format!("{}.{}", dist.dir, dist.ext);
    let src = paths.tmp.join(&file);

    if !src.exists() {
        crate::remote::download_dist(&map_version, &file, &src)?;
    }
    // dbg!(&src);
    // dbg!(&all);

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(time::Duration::from_millis(100));
    spinner.set_message("Extracting...");

    #[cfg(target_family = "unix")]
    let ok = {
        use std::process::Command;
        Command::new("tar")
            .arg("-xf")
            .arg(src)
            .arg("-C")
            .arg(&paths.all)
            .status()
            .is_ok_and(|s| s.success())
    };

    #[cfg(target_family = "windows")]
    let ok = sevenz_rust::decompress_file(src, &paths.all).is_ok();

    if !ok {
        anyhow::bail!("failed to extract: {}", version);
    }

    spinner.finish_with_message("Extracted.");

    if fs::rename(paths.all.join(dist.dir), dst).is_err() {
        anyhow::bail!("failed to install: {}", version);
    }
    println!("installed: {}", map_version);
    Ok(())
}
