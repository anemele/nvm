use crate::semver::map_versions;
use crate::semver::{VersionMap, VersionVec};
use serde::Deserialize;
// use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Index {
    pub version: String,
    // pub date: String,
    // pub files: Vec<String>,
    // pub npm: Option<String>,
    // pub v8: String,
    // pub uv: Option<String>,
    // pub zlib: Option<String>,
    // pub openssl: Option<String>,
    // pub modules: Option<String>,
    // pub lts: Value,
    // pub security: bool,
}

pub type Indexes = Vec<Index>;

fn get_node_url(path: &str) -> String {
    format!("https://nodejs.org/dist/{}", path)
}

fn get_index() -> anyhow::Result<Indexes> {
    let url = get_node_url("index.json");
    let res = tinyget::get(url)
        .with_header("User-Agent", "NVM Client")
        .send()?;

    let i: Indexes = serde_json::from_slice(res.as_bytes())?;
    Ok(i)
}

pub fn get_map_versions() -> anyhow::Result<(VersionMap, VersionVec)> {
    let indexes = get_index()?;

    let versions: Vec<String> = indexes
        .iter()
        .map(|index| index.version[1..].to_owned())
        .collect();

    Ok(map_versions(versions))
}

pub fn download_dist(url: &str, path: &Path) -> bool {
    let Ok(res) = tinyget::get(url)
        .with_header("User-Agent", "NVM Client")
        .send()
    else {
        return false;
    };

    if res.status_code >= 300 {
        return false;
    }

    let Ok(mut file) = File::create(&path) else {
        return false;
    };
    let Ok(size) = file.write(res.as_bytes()) else {
        return false;
    };

    size > 0
}
