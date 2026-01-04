use std::collections::HashSet;
use std::fs;

use colored::Colorize;
use dialoguer::Confirm;
use dialoguer::MultiSelect;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use nvm_common::consts;
use nvm_common::dist;
use nvm_common::path;
use nvm_common::semver;
use nvm_local as local;
use nvm_remote as remote;

#[inline]
fn print_env(key: &str, value: &str) {
    #[cfg(unix)]
    println!("export {key}={value}");
    #[cfg(windows)]
    println!("set {key}={value}");
}

pub fn cmd_env() -> anyhow::Result<()> {
    print_env(consts::KEY_NVM_DIST_MIRROR, consts::NVM_DIST_MIRROR);

    Ok(())
}

pub fn cmd_list_local() -> anyhow::Result<()> {
    let local_versions = local::query()?;
    // dbg!(&local_versions);

    if local_versions.versions.is_empty() {
        println!("Nothing available, install first.");
        return Ok(());
    }

    // See issue: https://github.com/colored-rs/colored/issues/76#issuecomment-616869300
    // and the solution: https://docs.rs/colored/1.9.3/x86_64-pc-windows-msvc/colored/control/fn.set_virtual_terminal.html
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    for v in local_versions.versions {
        if v == local_versions.current {
            println!("* {}", v.green())
        } else {
            println!("  {v}")
        }
    }

    Ok(())
}

pub fn cmd_list_remote(prefix: Option<String>) -> anyhow::Result<()> {
    let (map, mut vec, _) = remote::get_versions()?;

    let local_versions = local::query().unwrap_or_default();

    let mut local_versions_set = HashSet::new();
    for v in local_versions.versions {
        local_versions_set.insert(v);
    }

    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    if let Some(prefix) = prefix {
        vec.retain(|v| v.starts_with(&prefix));
        if vec.is_empty() {
            println!("No versions found with prefix: {prefix}");
            return Ok(());
        }
    }

    for key in vec {
        let v = map[&key].to_string();
        if v == local_versions.current {
            println!("* {:7} =>  {}", key.green(), v.green())
        } else if local_versions_set.contains(&v) {
            println!("  {:7} =>  {}", key.green(), v.green())
        } else {
            println!("  {key:7} =>  {v}")
        }
    }

    Ok(())
}

pub fn cmd_use(version: Option<String>) -> anyhow::Result<()> {
    let local_versions = local::query()?;

    let version = match version {
        Some(s) => s,
        None => {
            let default_pos = local_versions
                .versions
                .iter()
                .position(|s| s == &local_versions.current)
                .unwrap_or_default();
            let sel = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a version")
                .items(&local_versions.versions)
                .default(default_pos)
                .interact()?;
            local_versions.versions[sel].clone()
        }
    };

    let (map, _) = semver::map_versions(&local_versions.versions);
    let map_version = match map.get(&version) {
        Some(s) => s.to_string(),
        None => version.clone(),
    };

    if map_version == local_versions.current {
        println!("{map_version} is in use");
        return Ok(());
    }

    let paths = path::get_paths()?;
    let dist = dist::get_dist(&map_version).dir;
    let want = paths.home.join(format!("v{map_version}")).join(dist);

    if !want.exists() {
        println!("Not found: {version}");
        return Ok(());
    }

    let current = paths.current;

    #[cfg(windows)]
    let ok = {
        if current.exists() && fs::remove_dir(&current).is_err() {
            false
        } else {
            junction::create(want, current).is_ok()
        }
    };

    #[cfg(unix)]
    let ok = {
        use std::os::unix::fs::symlink;
        if current.exists() && fs::remove_file(&current).is_err() {
            false
        } else {
            symlink(want, current).is_ok()
        }
    };

    if ok {
        println!("Use {map_version}");
    } else {
        anyhow::bail!("Fail to use {version}");
    }

    Ok(())
}

pub fn cmd_install(version: &str) -> anyhow::Result<()> {
    let (map, _, vec) = remote::get_versions()?;
    let mapped_version = match map.get(version) {
        Some(v) => v.to_string(),
        None => {
            if !vec.contains(&version.to_string()) {
                anyhow::bail!("version not found: {version}");
            }
            eprintln!(
                "{}: {} is not a latest version",
                "WARNING".yellow(),
                version
            );
            version.to_string()
        }
    };

    let paths = path::get_paths()?;

    // check if the version is already installed
    let dest = paths.home.join(format!("v{mapped_version}"));
    if !dest.exists() {
        fs::create_dir(&dest)?;
    }
    if path::is_valid_nodejs(&dest) {
        println!("Exists: {}", dest.display());
        return Ok(());
    }

    // get distribution info
    let dist = dist::get_dist(&mapped_version);
    let file = format!("{}.{}", dist.dir, dist.ext);

    let cache = paths.cache.join(&file);

    if !cache.exists() || !local::check_sha256sum(&paths.cache, &file)? {
        remote::download_dist(&mapped_version, &file, &paths.cache)?;
    }

    local::extract_dist(&cache, &dest)?;

    println!("Installed: {mapped_version}");
    Ok(())
}

pub fn cmd_uninstall(versions: Vec<String>) -> anyhow::Result<()> {
    let local_versions = local::query()?;

    let versions: Vec<String> = if versions.is_empty() {
        let sels = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select versions to uninstall")
            .items(&local_versions.versions)
            .interact()?;
        if sels.is_empty() {
            println!("Nothing selected");
            return Ok(());
        }
        sels.iter()
            .map(|&i| local_versions.versions[i].clone())
            .collect()
    } else {
        let (map, _) = semver::map_versions(&local_versions.versions);
        versions
            .iter()
            .map(|v| match map.get(v) {
                Some(s) => s.to_string(),
                None => v.to_string(),
            })
            .collect()
    };

    for version in versions {
        let paths = path::get_paths()?;
        let want = paths.home.join(format!("v{version}"));
        if want.is_dir() {
            if fs::remove_dir_all(want).is_ok() {
                println!("Removed: {version}");
                if version == local_versions.current {
                    println!("{}: remove current version {}", "WARNING".yellow(), version);
                    let _ = fs::remove_dir(paths.current);
                }
            } else {
                eprintln!("Failed to remove: {version}")
            }
        } else {
            println!("Not found: {version}")
        }
    }

    Ok(())
}

pub fn cmd_clean(yes: bool) -> anyhow::Result<()> {
    let paths = path::get_paths()?;

    let confirmation = yes
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Clean cache?")
            .default(false)
            .interact()?;

    if !confirmation {
        println!("Canceled");
        return Ok(());
    }

    if fs::remove_dir_all(&paths.cache).is_err() {
        anyhow::bail!(
            "Failed to clean cache, do it manually. ({})",
            paths.cache.display()
        );
    }

    println!("Cache cleaned");
    Ok(())
}
