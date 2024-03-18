use crate::core::{get_path, query_local};
use colored::Colorize;

pub fn list_local() {
    let Some((all, bin, _)) = get_path() else {
        return;
    };

    let Some(local_versions) = query_local(&all, &bin) else {
        return;
    };

    if local_versions.versions.len() == 0 {
        println!("no available. install first.");
        return;
    }

    for node in local_versions.versions {
        if node == local_versions.current {
            println!("* {}", node.green())
        } else {
            println!("  {}", node)
        }
    }
}
