use std::fs;
use std::io::Read;
use std::path::Path;

use indicatif::ProgressBar;
use sha2::{Digest, Sha256};

use crate::consts::UNPACKED_SUCCESS_FILE;
use crate::utils;
use crate::utils::is_valid_nodejs;

#[derive(Default, Debug)]
pub struct LocalVersions {
    pub current: String,
    pub versions: Vec<String>,
}

fn get_ver_from_name(path: impl AsRef<Path>) -> Option<String> {
    let name = path.as_ref().file_name()?.to_str()?;
    name.split("-").nth(1).map(|s| s[1..].to_string())
}

fn get_current_version() -> anyhow::Result<String> {
    let paths = utils::get_paths()?;
    let link = fs::read_link(paths.current)?;
    if let Some(v) = get_ver_from_name(link) {
        Ok(v)
    } else {
        anyhow::bail!("failed to get current version");
    }
}

pub fn query() -> anyhow::Result<LocalVersions> {
    let paths = glob::glob(&format!(
        "{}/v*/node-v*",
        utils::get_paths()?.home.display()
    ))?;

    let mut versions = vec![];
    for path in paths {
        // dbg!(&path);
        let Ok(path) = path else {
            continue;
        };
        let Some(parent) = path.parent() else {
            continue;
        };
        // dbg!(&parent);
        if is_valid_nodejs(parent) {
            let Some(v) = get_ver_from_name(path) else {
                continue;
            };
            versions.push(v)
        }
    }

    let ret = LocalVersions {
        current: get_current_version().unwrap_or_default(),
        versions,
    };
    Ok(ret)
}

static CHUNK_SIZE: usize = 1024 * 4;
pub fn check_sha256sum(path: &Path, file: &str) -> anyhow::Result<bool> {
    let mut f = fs::File::open(path.join(file))?;
    let mut buf = [0u8; CHUNK_SIZE];
    let mut hasher = Sha256::new();
    loop {
        let n = f.read(&mut buf)?;
        hasher.update(&buf[..n]);
        if n < CHUNK_SIZE {
            break;
        }
    }
    let digest = format!("{:x}", hasher.finalize());
    let sha256_file = path.join(format!("{}.sha256", file));
    let sha256_str = fs::read_to_string(&sha256_file)?;
    Ok(digest == sha256_str.trim())
}

pub fn extract_dist(src: &Path, dest: &Path) -> anyhow::Result<()> {
    let spinner = ProgressBar::new_spinner();

    spinner.set_message("Extracting...");

    #[cfg(target_family = "unix")]
    let ok = {
        let file = fs::File::open(src)?;
        let reader = std::io::BufReader::new(file);
        let gz_reader = xz2::read::XzDecoder::new(reader);
        tar::Archive::new(gz_reader).unpack(dest).is_ok()
    };

    #[cfg(target_family = "windows")]
    let ok = sevenz_rust::decompress_file(src, dest).is_ok();

    let dot_file = dest.join(UNPACKED_SUCCESS_FILE);
    if !ok {
        if dot_file.exists() {
            let _ = fs::remove_file(&dot_file);
        }
        anyhow::bail!("failed to extract.");
    }
    if !dot_file.exists() {
        fs::File::create(dot_file)?;
    }

    spinner.finish_with_message("Extracted.");

    Ok(())
}
