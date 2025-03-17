use nvm_core::local;
use nvm_core::semver;
use nvm_core::utils;
use nvm_core::utils::get_dist;
use std::fs;

pub fn exec(version: &str) -> anyhow::Result<()> {
    let local_versions = local::query()?;
    let (map, _) = semver::map_versions(local_versions.versions);
    let map_version = match map.get(version) {
        Some(s) => s.to_string(),
        None => version.to_string(),
    };

    if map_version == local_versions.current {
        println!("current version is in use: {}", map_version);
        return Ok(());
    }

    let paths = utils::get_paths()?;
    let dist = get_dist(&map_version).dir;
    let want = paths.home.join(format!("v{}", map_version)).join(dist);

    if !want.exists() {
        println!("not found: {}", version);
        return Ok(());
    }

    #[cfg(target_family = "windows")]
    {
        if paths.current.exists() && fs::remove_dir_all(&paths.current).is_err() {
            anyhow::bail!("failed to remove link: {}", paths.current.display());
        }

        let Some(bin) = paths.current.to_str() else {
            anyhow::bail!(
                "failed to convert path to string: {}",
                paths.current.display()
            );
        };
        let Some(want) = want.to_str() else {
            anyhow::bail!("failed to convert path to string: {}", want.display());
        };

        use std::process::Command;
        use std::process::Stdio;

        let status = Command::new("cmd.exe")
            .arg("/c")
            .arg("mklink")
            .arg("/j")
            .arg(bin)
            .arg(want)
            .stdout(Stdio::null())
            .status();
        if status.is_ok_and(|s| s.success()) {
            println!("use {}", map_version)
        } else {
            anyhow::bail!("fail to use {}", version);
        }
    }

    #[cfg(target_family = "unix")]
    {
        if paths.bin.exists() && fs::remove_file(&paths.bin).is_err() {
            anyhow::bail!("failed to remove link: {}", paths.bin.display());
        }

        use std::os::unix::fs::symlink;

        if symlink(want, paths.bin).is_ok() {
            println!("use {}", map_version)
        } else {
            anyhow::bail!("fail to use {}", version);
        }
    }

    Ok(())
}
