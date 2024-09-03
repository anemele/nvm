use crate::semver::map_versions;
use crate::semver::{VersionMap, VersionVec};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use serde::Deserialize;
use std::io::Write;
// use serde_json::Value;
use std::path::Path;
use std::{fs, time};

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
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(time::Duration::from_millis(100));
    spinner.set_message("Fetching index.json");

    let i = Client::new()
        .get(url)
        .header("User-Agent", "NVM Client")
        .timeout(time::Duration::from_secs(10))
        .send()?
        .json::<Indexes>()?;

    spinner.finish_with_message("Read index.json, done.");

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

static CHUNK_SIZE: u64 = 1024 * 1024;

pub fn download_dist(url: &str, path: &Path) -> anyhow::Result<()> {
    let client = Client::new();

    let res = client
        .head(url)
        .header("User-Agent", "NVM Client")
        .timeout(time::Duration::from_secs(10))
        .send()?;
    let headers = res.headers();
    let content_length = headers
        .get(CONTENT_LENGTH)
        .unwrap()
        .to_str()?
        .parse::<u64>()?;
    let chunk_num = content_length / CHUNK_SIZE + 1;

    let mut out_file = fs::File::create(path)?;

    let pb = ProgressBar::new(content_length);
    pb.set_style(
        ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));
    pb.enable_steady_tick(time::Duration::from_millis(100));

    for i in 0..chunk_num {
        let start = i * CHUNK_SIZE;
        let end = (i + 1) * CHUNK_SIZE;
        let range = format!("bytes={}-{}", start, end - 1);
        let buf = client
            .get(url)
            .header("User-Agent", "NVM Client")
            .header("Range", range)
            .timeout(time::Duration::from_secs(10))
            .send()?
            .bytes()?;
        out_file.write(&buf)?;
        pb.inc(buf.len() as u64);
    }

    pb.finish();

    Ok(())
}
