use super::get_node_url;
use super::map_versions;
use super::{VersionMap, VersionVec};
use reqwest::{blocking::Client, Result};
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

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

fn get_index() -> Result<Indexes> {
    let url = get_node_url("index.json");
    Client::new()
        .get(url)
        .header("User-Agent", "NVM Client")
        .send()?
        .json::<Indexes>()
}

#[test]
fn test_get_index() {
    match get_index() {
        Ok(indexes) => {
            assert!(indexes.len() > 0);
            assert!(indexes[0].files.len() > 0);
        }
        Err(e) => assert_eq!("", e.to_string()),
    }
}

pub fn get_map_versions() -> Option<(VersionMap, VersionVec)> {
    let Ok(indexes) = get_index() else {
        return None;
    };

    let mut versions = vec![];
    for index in indexes {
        versions.push(index.version[1..].to_owned())
    }

    Some(map_versions(versions))
}

pub fn get_dist(url: &str, path: &Path) -> bool {
    let mut res = match Client::new()
        .get(url)
        .header("User-Agent", "NVM Client")
        .send()
    {
        Err(_) => return false,
        Ok(res) => res,
    };

    if !res.status().is_success() {
        return false;
    }

    let mut buf = vec![];
    match res.read_to_end(&mut buf) {
        Err(_) => return false,
        Ok(_) => match File::create(&path) {
            Err(_) => false,
            Ok(mut file) => match file.write(buf.as_slice()) {
                Ok(size) => size > 0,
                Err(_) => false,
            },
        },
    }
}
