use crate::consts::{NODE_CACHE, NODE_CURRENT, NODE_HOME, UNPACKED_SUCCESS_FILE};
use std::env::consts::{ARCH, OS};
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
        anyhow::bail!("failed to get NODE_HOME: ~/{}", NODE_HOME)
    }
}

#[derive(Debug)]
pub struct NodePaths {
    pub home: PathBuf,
    pub cache: PathBuf,
    pub current: PathBuf,
}

pub fn get_paths() -> anyhow::Result<NodePaths> {
    let home = get_node_home()?;
    let cache = home.join(NODE_CACHE);
    let current = home.join(NODE_CURRENT);

    for path in vec![&home, &cache] {
        if path.exists() {
            continue;
        }
        if fs::create_dir(path).is_err() {
            anyhow::bail!("failed to create dir: {}", path.display());
        }
    }

    let paths = NodePaths {
        home,
        cache,
        current,
    };
    Ok(paths)
}

pub struct Dist {
    pub dir: String,
    pub ext: String,
}

pub fn get_dist(version: &str) -> Dist {
    // node-v{v}-{os}-{arch}.{ext}
    // v:   {semver}
    // os:  win, linux, darwin
    // arc: x64, x86, arm64, ...
    // ext: zip, 7z, tar.gz, tar.xz

    let os = match OS {
        "windows" => "win",
        "macos" => "darwin",
        x => x,
    };
    // use .7z on Windows, tar.xz on *NIX for a smaller size.
    let ext = match os {
        "win" => "7z",
        _ => "tar.xz",
    };
    let arch = match ARCH {
        "x86_64" => "x64",
        "arm" => "arm64",
        x => x,
    };
    let dir = format!("node-v{version}-{os}-{arch}");
    Dist {
        dir,
        ext: ext.to_string(),
    }
}

pub fn is_valid_nodejs(path: &PathBuf) -> bool {
    path.join(UNPACKED_SUCCESS_FILE).exists()
}
