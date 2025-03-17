use clap::Parser;
use nvm_cmd::*;

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
    Use { version: String },

    /// Install some version
    #[clap(aliases =["i", "add"])]
    Install { version: String },

    /// Uninstall some version
    #[clap(alias = "rm")]
    Uninstall { version: String },

    /// Clean cache
    Clean,
}

fn main() -> anyhow::Result<()> {
    use Cli::*;
    match Cli::parse() {
        List => cmd_list_local::exec(),
        ListRemote => cmd_list_remote::exec(),
        Use { version } => cmd_use::exec(&version),
        Install { version } => cmd_install::exec(&version),
        Uninstall { version } => cmd_uninstall::exec(&version),
        Clean => cmd_clean::exec(),
    }
}
