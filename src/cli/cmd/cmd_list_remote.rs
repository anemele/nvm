use crate::core::get_index;
use semver::Version;
use std::str::FromStr;

pub fn cmd_list_remote() {
    if let Ok(indexes) = get_index() {
        let num = 3;
        let mut ctr = 0;
        let mut major = 0;
        for index in indexes {
            let version = Version::from_str(&index.version[1..]).unwrap();
            if version.major % 2 == 1 {
                continue;
            }
            if version.major == major {
                continue;
            }
            major = version.major;
            println!("{}", version);
            ctr += 1;
            if ctr == num {
                break;
            }
        }
    }
}
