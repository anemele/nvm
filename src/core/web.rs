use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use super::{get_node_url, Indexes};
use reqwest;
use reqwest::{blocking::Client, Result};

pub fn get_index() -> Result<Indexes> {
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
