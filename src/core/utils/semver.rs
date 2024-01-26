use semver::Version;
use std::collections::HashMap;
use std::str::FromStr;

pub type VersionMap = HashMap<String, String>;
pub type VersionVec = Vec<String>;
pub fn map_versions(versions: Vec<String>) -> (VersionMap, VersionVec) {
    let mut map = VersionMap::new();
    let mut vec = VersionVec::new();

    let mut major = 0;
    let mut minor = 0;
    for version in versions {
        if let Ok(version) = Version::from_str(&version) {
            if version.major != major {
                major = version.major;
                let key = major.to_string();
                map.insert(key.clone(), version.to_string());
                vec.push(key);
            }
            if version.minor != minor {
                minor = version.minor;
                let key = format!("{}.{}", major, minor);
                map.insert(key.clone(), version.to_string());
                vec.push(key);
            }
        }
    }

    vec.reverse();
    (map, vec)
}
