use colored::Colorize;
use std::collections::HashSet;

use crate::core::{get_map_versions, get_path, query_local, LocalVersions};

pub fn list_remote() {
    let Some((map, vec)) = get_map_versions() else {
        return;
    };

    let local_versions = if let Some((all, bin, _)) = get_path() {
        if let Some(local_versions) = query_local(&all, &bin) {
            local_versions
        } else {
            LocalVersions::default()
        }
    } else {
        LocalVersions::default()
    };

    let mut local_versions_set = HashSet::new();
    for v in local_versions.versions {
        local_versions_set.insert(v);
    }

    for key in vec {
        let v = map[&key].to_string();
        if v == local_versions.current {
            println!("* {:7}=>  {}", key.green(), v.green())
        } else if local_versions_set.take(&v).is_some() {
            println!("  {:7}=>  {}", key.green(), v.green())
        } else {
            println!("  {:7}=>  {}", key, v)
        }
    }
}
