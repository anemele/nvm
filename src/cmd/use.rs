use crate::local::query_local;
use crate::semver::map_versions;
use crate::utils::get_path;
use std::fs;

pub fn exec(version: &str) {
    let Some((all, bin, _)) = get_path() else {
        return;
    };

    let Some(local_versions) = query_local(&all) else {
        return;
    };

    let (map, _) = map_versions(local_versions.versions);

    let map_version = match map.get(version) {
        Some(s) => s.to_string(),
        None => version.to_string(),
    };

    if map_version == local_versions.current {
        println!("current version is in use: {}", map_version);
        return;
    }

    let want = all.join(format!("v{}", map_version));
    #[cfg(target_family = "unix")]
    let want = want.join("bin");

    if !want.exists() {
        println!("not found: {}", version);
        return;
    }

    #[cfg(target_family = "windows")]
    {
        if bin.exists() {
            if fs::remove_dir(&bin).is_err() {
                eprintln!("failed to remove link: {}", bin.display());
                return;
            }
        }

        // This method requires run as admin
        // use std::os::windows::fs::symlink_dir;
        // if let Err(e) = symlink_dir(want, bin) {
        //     eprintln!("{}", e)
        // } else {
        //     println!("use {}", map_version)
        // }

        use std::process::Command;
        use std::process::Stdio;

        let status = Command::new("cmd.exe")
            .arg("/c")
            .arg("mklink")
            .arg("/j")
            .arg(bin.to_str().unwrap())
            .arg(want.to_str().unwrap())
            .stdout(Stdio::null())
            .status();
        if status.is_ok_and(|s| s.success()) {
            println!("use {}", map_version)
        } else {
            println!("fail to use {}", version)
        }
    }

    #[cfg(target_family = "unix")]
    {
        if bin.exists() {
            if fs::remove_file(&bin).is_err() {
                eprintln!("failed to remove link: {}", bin.display());
                return;
            }
        }

        use std::os::unix::fs::symlink;

        if symlink(want, bin).is_ok() {
            println!("use {}", map_version)
        } else {
            println!("fail to use {}", version)
        }
    }
}
