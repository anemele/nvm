use clap::Parser;
mod cli;
use cli::Cli;
use cli::Cli::*;

use nvm_cmd::*;

fn main() -> anyhow::Result<()> {
    match Cli::parse() {
        List => cmd_list_local::exec(),
        ListRemote => cmd_list_remote::exec(),
        Use { version } => cmd_use::exec(&version),
        Install { version } => cmd_install::exec(&version),
        Uninstall { version } => cmd_uninstall::exec(&version),
        Clean => cmd_clean::exec(),
    }
}
