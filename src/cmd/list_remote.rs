use crate::local::{query_local, LocalVersions};
use crate::remote::get_map_versions;
use crate::utils::get_path;
use colored::Colorize;
use std::collections::HashSet;

pub fn exec() {
    let Some((map, vec)) = get_map_versions() else {
        return;
    };

    let local_versions = if let Some((all, _, _)) = get_path() {
        query_local(&all).unwrap_or_default()
    } else {
        LocalVersions::default()
    };

    let mut local_versions_set = HashSet::new();
    for v in local_versions.versions {
        local_versions_set.insert(v);
    }

    #[cfg(target_family = "windows")]
    {
        use colored::control::set_virtual_terminal;
        set_virtual_terminal(true).unwrap();
    }

    for key in vec {
        let v = map[&key].to_string();
        if v == local_versions.current {
            println!("* {:7}=>  {}", key.green(), v.green())
        } else if local_versions_set.get(&v).is_some() {
            println!("  {:7}=>  {}", key.green(), v.green())
        } else {
            println!("  {:7}=>  {}", key, v)
        }
    }
}
