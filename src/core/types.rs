use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Index {
    pub version: String,
    pub date: String,
    pub files: Vec<String>,
    pub npm: Option<String>,
    pub v8: String,
    pub uv: Option<String>,
    pub zlib: Option<String>,
    pub openssl: Option<String>,
    pub modules: Option<String>,
    pub lts: Value,
    pub security: bool,
}

pub type Indexes = Vec<Index>;
