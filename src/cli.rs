use clap::Parser;

#[derive(Parser)]
#[clap(name = "nvm", author, version, about = "Nodejs Version Manager")]
pub enum Cli {
    /// List all installed nodejs
    #[clap(aliases =["ls", "l"])]
    List,

    /// List remote, by default only lts
    #[clap(alias = "lr")]
    ListRemote,

    /// Use some version
    Use { version: String },

    /// Install some version
    #[clap(aliases =["i", "in", "add"])]
    Install { version: String },

    /// Uninstall some version
    #[clap(alias = "rm")]
    Uninstall { version: String },

    /// Clean cache
    Clean,
}
