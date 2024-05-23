use crate::utils::get_path;
use anyhow::anyhow;
use std::fs;

pub fn exec() -> anyhow::Result<()> {
    let (_, _, tmp) = get_path()?;

    if fs::remove_dir_all(&tmp).is_err() {
        Err(anyhow!(
            "failed to clean cache, do it manually. ({})",
            tmp.display()
        ))
    } else {
        Ok(())
    }
}
