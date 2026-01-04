mod cmd;
mod core;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "nvm", author, version, about = "Nodejs Version Manager")]
enum Cli {
    /// List all installed nodejs
    #[clap(visible_alias = "ls")]
    List,

    /// List remote, by default only lts
    #[clap(visible_alias = "lr")]
    ListRemote { prefix: Option<String> },

    /// Use some version
    #[clap(visible_alias = "set")]
    Use { version: Option<String> },

    /// Install some version
    #[clap(visible_aliases =["i", "add"])]
    Install { version: String },

    /// Uninstall some version
    #[clap(visible_alias = "rm")]
    Uninstall { version: Vec<String> },

    /// Print env
    Env,

    /// Clean cache
    Clean {
        #[clap(long)]
        yes: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    use Cli::*;
    use cmd::*;
    match cli {
        List => cmd_list_local::run(),
        ListRemote { prefix } => cmd_list_remote::run(prefix),
        Use { version } => cmd_use::run(version),
        Install { version } => cmd_install::run(&version),
        Uninstall { version } => cmd_uninstall::run(version),
        Env => cmd_env::run(),
        Clean { yes } => cmd_clean::run(yes),
    }
}
