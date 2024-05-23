use crate::utils::get_path;
use std::fs;
use std::path::Path;

#[derive(Default, Debug)]
pub struct LocalVersions {
    pub current: String,
    pub versions: Vec<String>,
}

fn is_node_path(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();

    // These "unwraps" are f**king code
    let start_with_v = path.file_name().unwrap().to_str().unwrap().starts_with("v");

    #[cfg(target_family = "windows")]
    let node_exist = path.join("node.exe").is_file();
    #[cfg(target_family = "unix")]
    let node_exist = path.join("bin").join("node").is_file();

    start_with_v && node_exist
}

fn get_current_version() -> anyhow::Result<String> {
    let (_, bin, _) = get_path()?;
    let link = fs::read_link(bin)?;
    let v = link
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix("v")
        .unwrap();

    Ok(v.to_string())
}

pub fn query_local(all: impl AsRef<Path>) -> Option<LocalVersions> {
    let all = all.as_ref();

    let Ok(rd) = fs::read_dir(&all) else {
        eprintln!("fail to read path: {}", all.display());
        return None;
    };

    let mut versions = vec![];
    for r in rd {
        let Ok(de) = r else {
            continue;
        };
        let dir = de.path();
        if is_node_path(&dir) {
            let name = dir.file_name().unwrap().to_str().unwrap();
            versions.push(name[1..].to_string())
        }
    }

    Some(LocalVersions {
        current: get_current_version().unwrap_or(String::new()),
        versions,
    })
}
