use std::fs;

use dialoguer::{Confirm, theme::ColorfulTheme};

use crate::core::utils;

pub fn run(yes: bool) -> anyhow::Result<()> {
    let paths = utils::get_paths()?;

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
