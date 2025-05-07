use std::fs;

use clap::Parser;
use dialoguer::{Confirm, theme::ColorfulTheme};
use nvm_core::utils;

use super::Run;

#[derive(Parser, Debug)]
pub struct CleanCommand {
    #[clap(long)]
    yes: bool,
}

impl Run for CleanCommand {
    fn run(&self) -> anyhow::Result<()> {
        let paths = utils::get_paths()?;

        let confirmation = self.yes
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
}
