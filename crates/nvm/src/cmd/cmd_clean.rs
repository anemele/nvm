use std::fs;

use clap::Parser;
use nvm_core::utils;

use super::Run;

#[derive(Parser, Debug)]
pub struct CleanCommand;

impl Run for CleanCommand {
    fn run(&self) -> anyhow::Result<()> {
        let paths = utils::get_paths()?;

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
