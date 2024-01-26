use super::get_node_url;
use reqwest;
use reqwest::{blocking::Client, Result};
use semver::Version;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

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

pub fn get_dist(url: &str, path: &Path) -> bool {
    let mut res = match Client::new()
        .get(url)
        .header("User-Agent", "NVM Client")
        .send()
    {
        Err(_) => return false,
        Ok(res) => res,
    };
    let mut buf = vec![];
    match res.read_to_end(&mut buf) {
        Err(_) => return false,
        Ok(_) => {
            if let Ok(mut file) = File::create(&path) {
                match file.write(buf.as_slice()) {
                    Ok(u) => u > 0,
                    Err(_) => false,
                }
            } else {
                false
            }
        }
    }
}

pub type VersionMap = HashMap<String, String>;
pub type VersionVec = Vec<String>;
pub fn get_map_versions() -> (VersionMap, VersionVec) {
    let indexes = get_index().unwrap();

    let mut map = VersionMap::new();
    let mut vec = VersionVec::new();

    let mut major = 0;
    let mut minor = 0;
    for index in indexes {
        if let Ok(version) = Version::from_str(&index.version[1..]) {
            if version.major != major {
                major = version.major;
                let key = major.to_string();
                map.insert(key.clone(), version.to_string());
                vec.push(key);
            }
            if version.minor != minor {
                minor = version.minor;
                let key = format!("{}.{}", major, minor);
                map.insert(key.clone(), version.to_string());
                vec.push(key);
            }
        }
    }

    vec.reverse();
    (map, vec)
}
