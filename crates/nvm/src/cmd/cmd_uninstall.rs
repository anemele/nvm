use clap::Parser;
use nvm_core::local;
use nvm_core::semver;
use nvm_core::utils;
use std::fs;

use super::Run;

#[derive(Debug, Parser)]
pub struct UninstallCommand {
    version: String,
}

impl Run for UninstallCommand {
    fn run(&self) -> anyhow::Result<()> {
        let local_versions = local::query()?;
        // dbg!(&local_versions);
        let (map, _) = semver::map_versions(local_versions.versions);
        // dbg!(&map);
        let version = &self.version;
        let map_version = match map.get(version) {
            Some(v) => v.to_string(),
            None => version.to_string(),
        };
        // dbg!(&map_version);

        let paths = utils::get_paths()?;
        let want = paths.home.join(&format!("v{}", map_version));
        if want.is_dir() {
            if fs::remove_dir_all(want).is_ok() {
                println!("removed: {}", map_version);
                if map_version == local_versions.current {
                    let _ = fs::remove_dir(paths.current);
                }
            } else {
                eprintln!("failed to remove: {}", map_version)
            }
        } else {
            println!("not found: {}", version)
        }

        Ok(())
    }
}
