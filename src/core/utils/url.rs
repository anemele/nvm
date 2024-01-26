use super::{NODE_BASE_URL, NPM_BASE_URL};

pub fn get_node_url(path: &str) -> String {
    NODE_BASE_URL.to_string() + path
}

pub fn get_npm_url(path: &str) -> String {
    NPM_BASE_URL.to_string() + path
}
