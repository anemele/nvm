use std::fs;

use clap::Parser;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use nvm_core::local;
use nvm_core::semver;
use nvm_core::utils;
use nvm_core::utils::get_dist;

use super::Run;

#[derive(Debug, Parser)]
pub struct UseCommand {
    version: Option<String>,
}

impl Run for UseCommand {
    fn run(&self) -> anyhow::Result<()> {
        let local_versions = local::query()?;

        let version = match self.version {
            Some(ref s) => s.clone(),
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
            println!("{} is in use", map_version);
            return Ok(());
        }

        let paths = utils::get_paths()?;
        let dist = get_dist(&map_version).dir;
        let want = paths.home.join(format!("v{}", map_version)).join(dist);

        if !want.exists() {
            println!("Not found: {}", version);
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
            println!("Use {}", map_version);
        } else {
            anyhow::bail!("Fail to use {}", version);
        }

        Ok(())
    }
}
