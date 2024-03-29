use crate::consts::{NODE_ALL, NODE_BIN, NODE_HOME, NODE_TMP};
use std::fs;
use std::path::PathBuf;
// use serde_json::Value;
use homedir::get_my_home;

// pub fn is_lts(lts: Value) -> bool {
//     match lts {
//         Value::String(_) => true,
//         Value::Bool(b) => b,
//         _ => false,
//     }
// }

// #[test]
// fn test_is_lts() {
//     assert!(is_lts(Value::Bool(true)));
//     assert!(!is_lts(Value::Bool(false)));
//     assert!(is_lts(Value::String("Iron".to_string())));
// }

fn get_node_home() -> Option<PathBuf> {
    let Ok(home) = get_my_home() else {
        return None;
    };
    let Some(home) = home else {
        return None;
    };
    Some(home.join(NODE_HOME))
}

pub fn get_path() -> Option<(PathBuf, PathBuf, PathBuf)> {
    let Some(home) = get_node_home() else {
        eprintln!("failed to get NODE_HOME: ~/{}", NODE_HOME);
        return None;
    };
    let all = home.join(NODE_ALL);
    let tmp = home.join(NODE_TMP);

    for path in vec![&home, &all, &tmp] {
        if path.exists() {
            continue;
        }
        if fs::create_dir(path).is_err() {
            eprintln!("failed to create dir: {}", path.display());
            return None;
        }
    }

    let bin = home.join(NODE_BIN);

    Some((all, bin, tmp))
}
