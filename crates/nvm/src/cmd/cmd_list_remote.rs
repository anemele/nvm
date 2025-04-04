use clap::Parser;
use colored::Colorize;
use nvm_core::local;
use nvm_core::remote;
use std::collections::HashSet;

use super::Run;

#[derive(Debug, Parser)]
pub struct ListRemoteCommand;

impl Run for ListRemoteCommand {
    fn run(&self) -> anyhow::Result<()> {
        let (map, vec, _) = remote::get_versions()?;

        let local_versions = local::query().unwrap_or_default();

        let mut local_versions_set = HashSet::new();
        for v in local_versions.versions {
            local_versions_set.insert(v);
        }

        #[cfg(target_family = "windows")]
        {
            colored::control::set_virtual_terminal(true).unwrap();
        }

        for key in vec {
            let v = map[&key].to_string();
            if v == local_versions.current {
                println!("{}", format!("* {:7} =>  {}", key, v).green())
            } else if local_versions_set.get(&v).is_some() {
                println!("{}", format!("  {:7} =>  {}", key, v).green())
            } else {
                println!("  {:7} =>  {}", key, v)
            }
        }

        Ok(())
    }
}
