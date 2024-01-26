use crate::core::get_map_versions;

pub fn cmd_list_remote() {
    let (map, vec) = get_map_versions();
    for key in vec {
        println!("{}\t{}", key, map[&key])
    }
}
