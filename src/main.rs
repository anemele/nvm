mod cli;
mod cmd;
mod consts;
mod local;
mod remote;
mod semver;
mod utils;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    match Cli::parse() {
        Cli::List => cmd::list_local::exec(),
        Cli::ListRemote => cmd::list_remote::exec(),
        Cli::Use { version } => cmd::r#use::exec(&version),
        Cli::Install { version } => cmd::install::exec(&version),
        Cli::Uninstall { version } => cmd::uninstall::exec(&version),
        Cli::Clean => cmd::clean::exec(),
    }
}
