use std::fs;

use colored::Colorize;
use nvm_core::local;
use nvm_core::remote;
use nvm_core::utils;

pub(super) fn run(version: &str) -> anyhow::Result<()> {
    let (map, _, vec) = remote::get_versions()?;
    let mapped_version = match map.get(version) {
        Some(v) => v.to_string(),
        None => {
            if !vec.contains(&version.to_string()) {
                anyhow::bail!("version not found: {}", version);
            }
            eprintln!(
                "{}: {} is not a latest version",
                "WARNING".yellow(),
                version
            );
            version.to_string()
        }
    };

    let paths = utils::get_paths()?;

    // check if the version is already installed
    let dest = paths.home.join(format!("v{mapped_version}"));
    if !dest.exists() {
        fs::create_dir(&dest)?;
    }
    if utils::is_valid_nodejs(&dest) {
        println!("Exists: {}", dest.display());
        return Ok(());
    }

    // get distribution info
    let dist = utils::get_dist(&mapped_version);
    let file = format!("{}.{}", dist.dir, dist.ext);

    let cache = paths.cache.join(&file);

    if !cache.exists() || !local::check_sha256sum(&paths.cache, &file)? {
        remote::download_dist(&mapped_version, &file, &paths.cache)?;
    }

    local::extract_dist(&cache, &dest)?;

    println!("Installed: {mapped_version}");
    Ok(())
}
