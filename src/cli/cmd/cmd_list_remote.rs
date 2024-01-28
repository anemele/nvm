use crate::core::get_map_versions;

pub fn cmd_list_remote() {
    let Some((map, vec)) = get_map_versions() else {
        return;
    };
    for key in vec {
        println!("{}\t{}", key, map[&key])
    }
}
