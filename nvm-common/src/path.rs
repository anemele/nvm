use std::fs;
use std::path::{Path, PathBuf};

use crate::consts::{NODE_CACHE, NODE_CURRENT, NODE_HOME, UNPACKED_SUCCESS_FILE};

fn get_node_home() -> anyhow::Result<PathBuf> {
    if let Ok(Some(home)) = homedir::my_home() {
        Ok(home.join(NODE_HOME))
    } else {
        anyhow::bail!("failed to get NODE_HOME: ~/{NODE_HOME}")
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

    for path in [&home, &cache] {
        if !path.exists() && fs::create_dir(path).is_err() {
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

#[inline]
pub fn get_dot_path(path: &Path) -> PathBuf {
    path.join(UNPACKED_SUCCESS_FILE)
}

#[inline]
pub fn is_valid_nodejs(path: &Path) -> bool {
    path.join(UNPACKED_SUCCESS_FILE).exists()
}
