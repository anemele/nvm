use crate::core::consts;

#[inline]
fn print_env(key: &str, value: &str) {
    #[cfg(unix)]
    println!("{key}={value}");
    #[cfg(windows)]
    println!("set {key}={value}");
}

pub fn run() -> anyhow::Result<()> {
    print_env(consts::KEY_NVM_DIST_MIRROR, consts::NVM_DIST_MIRROR);

    Ok(())
}
