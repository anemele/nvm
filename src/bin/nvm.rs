use clap::Parser;
use nvm::cmd;

#[derive(Parser)]
#[clap(name = "nvm", author, version, about = "Nodejs Version Manager")]
enum Cli {
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

fn main() {
    match Cli::parse() {
        Cli::List => cmd::list_local(),
        Cli::ListRemote => cmd::list_remote(),
        Cli::Use { version } => cmd::r#use(&version),
        Cli::Install { version } => cmd::install(&version),
        Cli::Uninstall { version } => cmd::uninstall(&version),
    }
}
