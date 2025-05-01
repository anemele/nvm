pub(crate) const NODE_HOME: &str = ".nodejs";

pub(crate) const NODE_CACHE: &str = "cache";
pub(crate) const NODE_CURRENT: &str = "current";

pub(crate) const UNPACKED_SUCCESS_FILE: &str = ".unpacked-success";

pub const KEY_NVM_DIST_MIRROR: &str = "NVM_DIST_MIRROR";
pub const NVM_DIST_MIRROR: &str = "https://nodejs.org/dist/";

pub fn get_mirror() -> String {
    let Ok(mut mirror) = std::env::var(KEY_NVM_DIST_MIRROR) else {
        return NVM_DIST_MIRROR.to_string();
    };

    if !mirror.ends_with("/") {
        mirror += "/";
    }
    mirror
}
