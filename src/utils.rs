use crate::consts::{NODE_ALL, NODE_BIN, NODE_HOME, NODE_TMP};
use anyhow::anyhow;
use std::fs;
use std::path::PathBuf;
// use serde_json::Value;

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

fn get_node_home() -> anyhow::Result<PathBuf> {
    if let Ok(Some(home)) = homedir::get_my_home() {
        Ok(home.join(NODE_HOME))
    } else {
        Err(anyhow!("failed to get NODE_HOME: ~/{}", NODE_HOME))
    }
}

#[derive(Debug)]
pub struct NodePaths {
    pub all: PathBuf,
    pub bin: PathBuf,
    pub tmp: PathBuf,
}

pub fn get_paths() -> anyhow::Result<NodePaths> {
    let home = get_node_home()?;
    let all = home.join(NODE_ALL);
    let tmp = home.join(NODE_TMP);

    for path in vec![&home, &all, &tmp] {
        if path.exists() {
            continue;
        }
        if fs::create_dir(path).is_err() {
            return Err(anyhow!("failed to create dir: {}", path.display()));
        }
    }

    let bin = home.join(NODE_BIN);

    let paths = NodePaths { all, bin, tmp };
    Ok(paths)
}
