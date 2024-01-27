use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn get_node_url(path: &str) -> String {
    format!("https://nodejs.org/dist/{}", path)
}

pub fn get_dist_url(version: &str) -> String {
    format!(
        "https://nodejs.org/dist/v{0}/node-v{0}-win-x64.zip",
        version
    )
}

const NODE_HOME: &str = ".nodejs";

fn get_node_home() -> PathBuf {
    Path::new(&env::var("USERPROFILE").unwrap()).join(NODE_HOME)
}

#[test]
fn test_get_node_home() {
    let home = get_node_home();
    let home_str = home.as_os_str().to_str().unwrap().to_string();
    assert!(home_str.starts_with("C:\\Users\\"));
    assert!(home_str.ends_with(&format!("\\{}", NODE_HOME)))
}

const NODE_ALL: &str = "all";
const NODE_BIN: &str = "bin";
const NODE_TMP: &str = "tmp";

pub fn get_path() -> Option<(PathBuf, PathBuf, PathBuf)> {
    let home = get_node_home();
    if !home.exists() {
        if let Err(e) = fs::create_dir(&home) {
            eprintln!("{}", e);
            return None;
        }
    }

    let all = home.join(NODE_ALL);
    if !all.exists() {
        if let Err(e) = fs::create_dir(&all) {
            eprintln!("{}", e);
            return None;
        }
    }

    let tmp = home.join(NODE_TMP);
    if !tmp.exists() {
        if let Err(e) = fs::create_dir(&tmp) {
            eprintln!("{}", e);
            return None;
        }
    }

    let bin = home.join(NODE_BIN);

    Some((all, bin, tmp))
}
