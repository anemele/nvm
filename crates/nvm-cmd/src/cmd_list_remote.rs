use colored::Colorize;
use nvm_core::local::LocalVersions;
use nvm_core::local::query_local;
use nvm_core::remote::get_map_versions;
use nvm_core::utils::get_paths;
use std::collections::HashSet;

pub fn exec() -> anyhow::Result<()> {
    let (map, vec) = get_map_versions()?;

    let local_versions = if let Ok(paths) = get_paths() {
        query_local(&paths.all).unwrap_or_default()
    } else {
        LocalVersions::default()
    };

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
