use crate::utils::is_node_path;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct LocalVersions {
    pub current: String,
    pub versions: Vec<String>,
}

pub fn query_local(all: &Path, bin: &Path) -> Option<LocalVersions> {
    let Ok(rd) = fs::read_dir(&all) else {
        eprintln!("fail to read path: {}", all.display());
        return None;
    };

    let current = match bin.read_link() {
        Ok(link) => link.file_name().unwrap().to_str().unwrap()[1..].to_string(),
        Err(_) => String::new(),
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

    Some(LocalVersions { current, versions })
}
