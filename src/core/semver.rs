use semver::Version;
use std::collections::HashMap;
use std::str::FromStr;

pub type VersionMap = HashMap<String, Version>;
pub type VersionVec = Vec<String>;

pub fn map_versions(versions: Vec<String>) -> (VersionMap, VersionVec) {
    let mut map = VersionMap::new();
    let mut vec = VersionVec::new();

    for version in versions {
        let Ok(sv) = Version::from_str(&version) else {
            continue;
        };

        let major = sv.major.to_string();
        if !map.contains_key(&major) {
            map.insert(major.clone(), sv.clone());
            vec.push(major);
        } else if map[&major].lt(&sv) {
            map.insert(major, sv.clone());
        }

        let mm = format!("{}.{}", sv.major, sv.minor);
        if !map.contains_key(&mm) {
            map.insert(mm.clone(), sv);
            vec.push(mm);
        } else if map[&mm].lt(&sv) {
            map.insert(mm, sv);
        }
    }

    vec.reverse();
    (map, vec)
}
