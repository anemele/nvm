use std::fs;

pub fn exec(version: &str) -> anyhow::Result<()> {
    let paths = crate::utils::get_paths()?;

    let local_versions = crate::local::query_local(&paths.all)?;

    let (map, _) = crate::semver::map_versions(local_versions.versions);

    let map_version = match map.get(version) {
        Some(s) => s.to_string(),
        None => version.to_string(),
    };

    if map_version == local_versions.current {
        println!("current version is in use: {}", map_version);
        return Ok(());
    }

    let want = paths.all.join(format!("v{}", map_version));
    #[cfg(target_family = "unix")]
    let want = want.join("bin");

    if !want.exists() {
        println!("not found: {}", version);
        return Ok(());
    }

    #[cfg(target_family = "windows")]
    {
        if paths.bin.exists() && fs::remove_dir_all(&paths.bin).is_err() {
            anyhow::bail!("failed to remove link: {}", paths.bin.display());
        }

        let Some(bin) = paths.bin.to_str() else {
            anyhow::bail!("failed to convert path to string: {}", paths.bin.display());
        };
        let Some(want) = want.to_str() else {
            anyhow::bail!("failed to convert path to string: {}", want.display());
        };

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
