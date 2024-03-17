use crate::core::get_map_versions;

pub fn list_remote() {
    let Some((map, vec)) = get_map_versions() else {
        return;
    };
    for key in vec {
        println!("  {:7}=>  {}", key, map[&key])
    }
}
