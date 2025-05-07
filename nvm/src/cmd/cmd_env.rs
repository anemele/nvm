use clap::Parser;

use nvm_core::consts;

use super::Run;

#[derive(Debug, Parser)]
pub struct EnvCommand;

#[inline]
fn print_env(key: &str, value: &str) {
    #[cfg(unix)]
    println!("{}={}", key, value);
    #[cfg(windows)]
    println!("set {}={}", key, value);
}

impl Run for EnvCommand {
    fn run(&self) -> anyhow::Result<()> {
        print_env(consts::KEY_NVM_DIST_MIRROR, consts::NVM_DIST_MIRROR);

        Ok(())
    }
}
