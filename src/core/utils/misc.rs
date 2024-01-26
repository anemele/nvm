use std::path::Path;

use serde_json::Value;

pub fn is_lts(lts: Value) -> bool {
    match lts {
        Value::String(_) => true,
        Value::Bool(b) => b,
        _ => false,
    }
}

pub fn is_node_path(path: &Path) -> bool {
    if let Some(name) = path.file_name() {
        name.to_str().unwrap().starts_with("v") && path.join("node.exe").is_file()
    } else {
        return false;
    }
}
