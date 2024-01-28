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

#[test]
fn test_map_versions() {
    let sample = vec![
        "20.0.0", "19.1.0", "19.0.1", "18.19.0", "18.18.2", "18.18.1", "18.18.0", "18.17.1",
    ];
    let sample: Vec<String> = sample.into_iter().map(|s| s.to_string()).collect();
    let (map, vec) = map_versions(sample);

    assert_eq!(vec.len(), 9);
    assert_eq!(
        vec,
        vec!["18.17", "18.18", "18.19", "18", "19.0", "19.1", "19", "20.0", "20",]
    );
    assert_eq!(map["20"], Version::new(20, 0, 0));
    assert_eq!(map["19"], Version::new(19, 1, 0));
    assert_eq!(map["18"], Version::new(18, 19, 0));
    assert_eq!(map["18.18"], Version::new(18, 18, 2));
}
