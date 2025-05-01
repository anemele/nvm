use crate::semver::{VersionMap, VersionVec, map_versions};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::Path;
// use serde_json::Value;
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
    crate::mirror::get_mirror() + path
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

    spinner.finish_with_message("Read index.json.");

    Ok(i)
}

pub fn get_versions() -> anyhow::Result<(VersionMap, VersionVec, VersionVec)> {
    let indexes = get_index()?;

    let versions: Vec<String> = indexes
        .iter()
        .map(|index| index.version[1..].to_owned())
        .collect();

    let (m, v) = map_versions(&versions);
    Ok((m, v, versions))
}

static CHUNK_SIZE: u64 = 1024 * 1024;

pub fn download_dist(version: &str, file: &str, cache: &Path) -> anyhow::Result<()> {
    let client = Client::new();

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(time::Duration::from_millis(100));

    spinner.set_message("Fetching Checksum ...");
    let url = get_node_url(&format!("v{}/SHASUMS256.txt", version));
    // dbg!(&url);
    let sha256_txt = client
        .get(url)
        .header("User-Agent", "NVM Client")
        .timeout(time::Duration::from_secs(10))
        .send()?
        .text()?;
    // dbg!(&file);
    // dbg!(&sha256_txt);
    let Some(sha256_line) = sha256_txt.lines().find(|line| line.ends_with(file)) else {
        anyhow::bail!("not found SHASUMS256.txt for {}.", file);
    };
    let Some(sha256_expected) = sha256_line.split_whitespace().next() else {
        anyhow::bail!("not found checksum for {}.", file);
    };
    fs::write(
        cache.join(format!("{}.sha256", file)),
        sha256_expected.as_bytes(),
    )?;
    spinner.finish_and_clear();

    spinner.set_message("Fetching Head Info ...");
    let url = get_node_url(&format!("v{}/{}", version, file));
    let res = client
        .head(&url)
        .header("User-Agent", "NVM Client")
        .timeout(time::Duration::from_secs(10))
        .send()?;
    let headers = res.headers();
    let content_length = headers
        .get(CONTENT_LENGTH)
        .unwrap()
        .to_str()?
        .parse::<u64>()?;
    spinner.finish_and_clear();

    let mut cache_file = fs::File::create(cache.join(file))?;
    let mut hasher = Sha256::new();

    let pb = ProgressBar::new(content_length);
    pb.set_style(
        ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));
    pb.enable_steady_tick(time::Duration::from_millis(100));

    let mut start = 0;
    while start < content_length {
        let end = start + CHUNK_SIZE;
        let range = format!("bytes={}-{}", start, end);
        let buf = client
            .get(&url)
            .header("User-Agent", "NVM Client")
            .header("Range", range)
            .timeout(time::Duration::from_secs(10))
            .send()?
            .bytes()?;
        cache_file.write_all(&buf)?;
        hasher.update(&buf);
        pb.inc(buf.len() as u64);
        start = end + 1;
    }

    pb.finish_and_clear();

    let sha256_actual = format!("{:x}", hasher.finalize());
    if sha256_expected == sha256_actual {
        println!("Checksum verified.");
    } else {
        anyhow::bail!("Checksum mismatched.");
    }

    Ok(())
}
