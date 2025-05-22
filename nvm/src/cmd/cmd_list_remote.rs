use colored::Colorize;
use nvm_core::local;
use nvm_core::remote;
use std::collections::HashSet;

pub(super) fn run(prefix: Option<String>) -> anyhow::Result<()> {
    let (map, mut vec, _) = remote::get_versions()?;

    let local_versions = local::query().unwrap_or_default();

    let mut local_versions_set = HashSet::new();
    for v in local_versions.versions {
        local_versions_set.insert(v);
    }

    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    if let Some(prefix) = prefix {
        vec.retain(|v| v.starts_with(&prefix));
        if vec.is_empty() {
            println!("No versions found with prefix: {}", prefix);
            return Ok(());
        }
    }

    for key in vec {
        let v = map[&key].to_string();
        if v == local_versions.current {
            println!("* {:7} =>  {}", key.green(), v.green())
        } else if local_versions_set.contains(&v) {
            println!("  {:7} =>  {}", key.green(), v.green())
        } else {
            println!("  {:7} =>  {}", key, v)
        }
    }

    Ok(())
}
