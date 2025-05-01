use clap::Parser;
use nvm_core::local;
use nvm_core::semver;
use nvm_core::utils;
use nvm_core::utils::get_dist;

use super::Run;

#[derive(Debug, Parser)]
pub struct UseCommand {
    version: String,
}

impl Run for UseCommand {
    fn run(&self) -> anyhow::Result<()> {
        let local_versions = local::query()?;
        let (map, _) = semver::map_versions(&local_versions.versions);
        let version = &self.version;
        let map_version = match map.get(version) {
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
        let ok = { junction::delete(&current).is_ok() && junction::create(want, current).is_ok() };

        #[cfg(unix)]
        let ok = {
            use std::os::unix::fs::symlink;
            current.exists() && fs::remove_file(&current).is_err() && symlink(want, current).is_ok()
        };

        if ok {
            println!("Use {}", map_version)
        } else {
            anyhow::bail!("fail to use {}", version);
        }

        Ok(())
    }
}
