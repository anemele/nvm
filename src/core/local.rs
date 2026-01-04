use std::fs;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

use indicatif::ProgressBar;
use sha2::{Digest, Sha256};

use super::consts::UNPACKED_SUCCESS_FILE;
use super::utils;

#[derive(Default, Debug)]
pub struct LocalVersions {
    pub current: String,
    pub versions: Vec<String>,
}

fn get_ver_from_name(path: impl AsRef<Path>) -> Option<String> {
    path.as_ref()
        .file_name()?
        .to_str()?
        .split('-')
        .nth(1)
        .map(|s| s[1..].to_string())
}

#[test]
fn test_get_ver_from_name() {
    assert_eq!(
        get_ver_from_name("/home/unix/.nodejs/v18.20.8/node-v18.20.8-linux-x64").unwrap(),
        "18.20.8"
    );
}

fn get_current_version() -> anyhow::Result<String> {
    let paths = utils::get_paths()?;
    let link = fs::read_link(paths.current)?;
    get_ver_from_name(link).ok_or_else(|| anyhow::anyhow!("failed to get current version"))
}

pub fn query() -> anyhow::Result<LocalVersions> {
    let patterns = format!("{}/v*/node-v*", utils::get_paths()?.home.display());

    let versions = glob::glob(&patterns)?
        .filter_map(|path| path.ok())
        .filter(|path| path.parent().is_some_and(utils::is_valid_nodejs))
        .filter_map(get_ver_from_name)
        .collect();
    // dbg!(&versions);

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
    let sha256_file = path.join(format!("{file}.sha256"));
    let sha256_str = fs::read_to_string(&sha256_file)?;
    Ok(digest == sha256_str.trim())
}

pub fn extract_dist(src: &Path, dest: &Path) -> anyhow::Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));

    spinner.set_message(format!("Extracting {}", src.display()));

    #[cfg(unix)]
    let ok = {
        let file = fs::File::open(src)?;
        let reader = std::io::BufReader::new(file);
        let gz_reader = xz2::read::XzDecoder::new(reader);
        tar::Archive::new(gz_reader).unpack(dest).is_ok()
    };

    #[cfg(windows)]
    let ok = sevenz_rust::decompress_file(src, dest).is_ok();

    let dot_file = dest.join(UNPACKED_SUCCESS_FILE);
    if !ok {
        if dot_file.exists() {
            let _ = fs::remove_file(&dot_file);
        }
        anyhow::bail!("Failed to extract.");
    }
    if !dot_file.exists() {
        fs::File::create(dot_file)?;
    }

    spinner.finish_and_clear();

    Ok(())
}
