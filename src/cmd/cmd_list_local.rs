use crate::core::{get_path, query_local};

pub fn list_local() {
    let Some((all, bin, _)) = get_path() else {
        return;
    };

    let Some(local_versions) = query_local(&all, &bin) else {
        return;
    };

    let mut count = 0;
    for node in local_versions.versions {
        if node == local_versions.current {
            println!("\x1b[32m* {}\x1b[m", node)
        } else {
            println!("  {}", node)
        }
        count += 1;
    }
    if count == 0 {
        println!("no available. install first.")
    }
}
