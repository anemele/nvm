use crate::consts::{NODE_ALL, NODE_BIN, NODE_HOME, NODE_TMP};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
// use serde_json::Value;
use homedir::get_my_home;

pub fn get_node_url(path: &str) -> String {
    format!("https://nodejs.org/dist/{}", path)
}

pub fn get_dist_url(version: &str) -> String {
    format!(
        "https://nodejs.org/dist/v{0}/node-v{0}-win-x64.zip",
        version
    )
}

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

pub fn is_node_path(path: &Path) -> bool {
    path.file_name().unwrap().to_str().unwrap().starts_with("v") && path.join("node.exe").is_file()
}

fn get_node_home() -> PathBuf {
    get_my_home().unwrap().unwrap().join(NODE_HOME)
}

#[test]
fn test_get_node_home() {
    let home = get_node_home();
    let home_str = home.as_os_str().to_str().unwrap().to_string();
    assert!(home_str.starts_with("C:\\Users\\"));
    assert!(home_str.ends_with(&format!("\\{}", NODE_HOME)));
}

pub fn get_path() -> Option<(PathBuf, PathBuf, PathBuf)> {
    let home = get_node_home();
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
