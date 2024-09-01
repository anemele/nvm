use crate::semver::map_versions;
use crate::semver::{VersionMap, VersionVec};
use anyhow::anyhow;
use serde::Deserialize;
// use serde_json::Value;
use std::fs;
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

pub fn download_dist(url: &str, path: &Path) -> anyhow::Result<()> {
    let res = tinyget::get(url)
        .with_header("User-Agent", "NVM Client")
        .send()?;

    if res.status_code >= 300 {
        return Err(anyhow!("Failed to download {}: {}", url, res.status_code));
    }

    fs::write(path, res.as_bytes())?;
    Ok(())
}
