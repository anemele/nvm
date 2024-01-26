use crate::core::{get_path, query_local};

pub fn cmd_list() {
    let tmp = get_path();
    if tmp.is_none() {
        return;
    }

    let (all, bin, _) = tmp.unwrap();

    let tmp = query_local(&all, &bin);
    if tmp.is_none() {
        return;
    }

    let (current, versions) = tmp.unwrap();

    let mut count = 0;
    for node in versions {
        if node == current {
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
