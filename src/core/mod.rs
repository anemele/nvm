mod consts;
mod local;
mod remote;
mod semver;
mod utils;

use consts::*;
pub use local::{query_local, unzip};
pub use remote::{get_dist, get_map_versions};
pub use semver::{map_versions, VersionMap, VersionVec};
use utils::is_node_path;
pub use utils::{get_dist_url, get_node_url, get_path};
