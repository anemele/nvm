use crate::semver::map_versions;
use crate::semver::{VersionMap, VersionVec};
use crate::utils::get_node_url;
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tinyget;

#[derive(Debug, Deserialize)]
pub struct Index {
    pub version: String,
    pub date: String,
    pub files: Vec<String>,
    pub npm: Option<String>,
    pub v8: String,
    pub uv: Option<String>,
    pub zlib: Option<String>,
    pub openssl: Option<String>,
    pub modules: Option<String>,
    pub lts: Value,
    pub security: bool,
}

pub type Indexes = Vec<Index>;

fn get_index() -> Option<Indexes> {
    let url = get_node_url("index.json");
    let Ok(res) = tinyget::get(url)
        .with_header("User-Agent", "NVM Client")
        .send()
    else {
        return None;
    };
    let Ok(i) = serde_json::from_slice::<Indexes>(res.as_bytes()) else {
        return None;
    };
    Some(i)
}

pub fn get_map_versions() -> Option<(VersionMap, VersionVec)> {
    let Some(indexes) = get_index() else {
        return None;
    };

    let mut versions = vec![];
    for index in indexes {
        versions.push(index.version[1..].to_owned())
    }

    Some(map_versions(versions))
}

pub fn get_dist(url: &str, path: &Path) -> bool {
    let res = match tinyget::get(url)
        .with_header("User-Agent", "NVM Client")
        .send()
    {
        Err(_) => return false,
        Ok(res) => res,
    };

    if res.status_code >= 300 {
        return false;
    }

    match File::create(&path) {
        Err(_) => false,
        Ok(mut file) => match file.write(res.as_bytes()) {
            Ok(size) => size > 0,
            Err(_) => false,
        },
    }
}
