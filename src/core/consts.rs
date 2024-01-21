use std::{
    env,
    path::{Path, PathBuf},
};

pub const NODE_BASE_URL: &str = "https://nodejs.org/dist/";
pub const NPM_BASE_URL: &str = "https://github.com/npm/cli/archive/";

pub fn get_dist_url(version: &str) -> String {
    format!(
        "https://nodejs.org/dist/v{0}/node-v{0}-win-x64.zip",
        version
    )
}

const NODE_HOME: &str = ".nodejs";
pub const NODE_ALL: &str = "all";
pub const NODE_BIN: &str = "bin";
pub const NODE_TMP: &str = "tmp";

pub fn get_node_home() -> PathBuf {
    Path::new(&env::var("USERPROFILE").unwrap()).join(NODE_HOME)
}

#[test]
fn test_get_node_home() {
    let home = get_node_home();
    let home_str = home.as_os_str().to_str().unwrap().to_string();
    assert!(home_str.starts_with("C:\\Users\\"));
    assert!(home_str.ends_with(&format!("\\{}", NODE_HOME)))
}
