use std::fs;

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use crate::core::local;
use crate::core::semver;
use crate::core::utils;
use crate::core::utils::get_dist;

pub fn run(version: Option<String>) -> anyhow::Result<()> {
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

    let paths = utils::get_paths()?;
    let dist = get_dist(&map_version).dir;
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
