use std::env;

const INITIAL_MIRROR: &str = "https://nodejs.org/dist/";

pub fn get_mirror() -> String {
    let Ok(mut mirror) = env::var("NVM_DIST_MIRROR") else {
        return INITIAL_MIRROR.to_string();
    };

    if !mirror.ends_with("/") {
        mirror += "/";
    }
    mirror
}
