use std::fs;

use nvm_core::utils;

pub fn exec() -> anyhow::Result<()> {
    let paths = utils::get_paths()?;

    if fs::remove_dir_all(&paths.cache).is_err() {
        anyhow::bail!(
            "failed to clean cache, do it manually. ({})",
            paths.cache.display()
        );
    }

    println!("cache cleaned");
    Ok(())
}
