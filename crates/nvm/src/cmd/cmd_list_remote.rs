use clap::Parser;
use colored::Colorize;
use nvm_core::local;
use nvm_core::remote;
use std::collections::HashSet;

use super::Run;

#[derive(Debug, Parser)]
pub struct ListRemoteCommand {
    /// Filter by prefix
    prefix: Option<String>,
}

impl Run for ListRemoteCommand {
    fn run(&self) -> anyhow::Result<()> {
        let (map, mut vec, _) = remote::get_versions()?;

        let local_versions = local::query().unwrap_or_default();

        let mut local_versions_set = HashSet::new();
        for v in local_versions.versions {
            local_versions_set.insert(v);
        }

        #[cfg(windows)]
        colored::control::set_virtual_terminal(true).unwrap();

        if let Some(prefix) = &self.prefix {
            vec.retain(|v| v.starts_with(prefix));
            if vec.is_empty() {
                println!("No versions found with prefix: {}", prefix);
                return Ok(());
            }
        }

        for key in vec {
            let v = map[&key].to_string();
            if v == local_versions.current {
                println!("{}", format!("* {:7} =>  {}", key, v).green())
            } else if local_versions_set.contains(&v) {
                println!("{}", format!("  {:7} =>  {}", key, v).green())
            } else {
                println!("  {:7} =>  {}", key, v)
            }
        }

        Ok(())
    }
}
