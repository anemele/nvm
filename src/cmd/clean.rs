use crate::utils::get_paths;
use anyhow::anyhow;
use std::fs;

pub fn exec() -> anyhow::Result<()> {
    let paths = get_paths()?;

    if fs::remove_dir_all(&paths.tmp).is_err() {
        return Err(anyhow!(
            "failed to clean cache, do it manually. ({})",
            paths.tmp.display()
        ));
    }

    println!("cache cleaned");
    Ok(())
}
