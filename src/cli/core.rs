use super::cmd::{cmd_install, cmd_list, cmd_list_remote, cmd_uninstall, cmd_use};
use clap::Parser;

#[derive(Parser)]
#[clap(name = "nvm", author, version, about = "Nodejs Version Manager")]
pub enum Cli {
    /// List all installed nodejs
    #[clap(alias = "ls")]
    List,
    /// List remote, by default only lts
    #[clap(alias = "lr")]
    ListRemote,
    /// Use some version
    Use {
        #[arg(help = "version")]
        version: String,
    },
    /// Install some version
    #[clap(alias = "i")]
    Install { version: String },
    /// Uninstall some version
    #[clap(alias = "rm")]
    Uninstall { version: String },
}

pub fn run() {
    match Cli::parse() {
        Cli::List => cmd_list(),
        Cli::ListRemote => cmd_list_remote(),
        Cli::Use { version } => cmd_use(&version),
        Cli::Install { version } => cmd_install(&version),
        Cli::Uninstall { version } => cmd_uninstall(&version),
    }
}
