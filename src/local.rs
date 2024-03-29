use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct LocalVersions {
    pub current: String,
    pub versions: Vec<String>,
}

fn is_node_path(path: &Path) -> bool {
    // These "unwraps" are f**king code
    let start_with_v = path.file_name().unwrap().to_str().unwrap().starts_with("v");

    #[cfg(target_family = "windows")]
    let node_exist = path.join("node.exe").is_file();
    #[cfg(target_family = "unix")]
    let node_exist = path.join("bin").join("node").is_file();

    start_with_v && node_exist
}

pub fn query_local(all: &Path, bin: &Path) -> Option<LocalVersions> {
    let Ok(rd) = fs::read_dir(&all) else {
        eprintln!("fail to read path: {}", all.display());
        return None;
    };

    let current = match bin.read_link() {
        Err(_) => String::new(),
        Ok(link) => {
            #[cfg(target_family = "unix")]
            let link = {
                let mut ancestors = link.ancestors();
                ancestors.next();
                ancestors.next()
            }
            .unwrap();

            // These f**king unwraps :)
            link.file_name().unwrap().to_str().unwrap()[1..].to_string()
        }
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
