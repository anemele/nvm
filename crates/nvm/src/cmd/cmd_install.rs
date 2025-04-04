use std::fs;

use clap::Parser;
use nvm_core::local;
use nvm_core::remote;
use nvm_core::utils;

use super::Run;

#[derive(Parser, Debug)]
pub struct InstallCommand {
    version: String,
}

impl Run for InstallCommand {
    fn run(&self) -> anyhow::Result<()> {
        let (map, _, vec) = remote::get_versions()?;
        let mapped_version = match map.get(&self.version) {
            Some(v) => v.to_string(),
            None => {
                if !vec.contains(&self.version) {
                    anyhow::bail!("{} is not a valid version", &self.version);
                }
                eprintln!("WARNING: {} is not a MAIN version", &self.version);
                self.version.clone()
            }
        };

        let paths = utils::get_paths()?;

        // check if the version is already installed
        let dest = paths.home.join(format!("v{}", mapped_version));
        if !dest.exists() {
            fs::create_dir(&dest)?;
        }
        if utils::is_valid_nodejs(&dest) {
            println!("exists: {}", dest.display());
            return Ok(());
        }

        // get distribution info
        let dist = utils::get_dist(&mapped_version);
        let file = format!("{}.{}", dist.dir, dist.ext);

        let cache = paths.cache.join(&file);
        // check if the version is already in cache
        if cache.exists() {
            println!("found cache: {}", cache.display());
            if local::check_sha256sum(&paths.cache, &file)? {
                println!("checksum verified.");
                local::extract_dist(&cache, &dest)?;
                println!("installed: {}", mapped_version);
                return Ok(());
            }
        }
        // if not, download the distribution
        remote::download_dist(&mapped_version, &file, &paths.cache)?;
        // extract the distribution
        local::extract_dist(&cache, &dest)?;

        println!("installed: {}", mapped_version);
        Ok(())
    }
}
