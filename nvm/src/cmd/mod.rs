use clap::Parser;
use cmd_clean::CleanCommand;
use cmd_env::EnvCommand;
use cmd_install::InstallCommand;
use cmd_list_local::ListLocalCommand;
use cmd_list_remote::ListRemoteCommand;
use cmd_uninstall::UninstallCommand;
use cmd_use::UseCommand;

mod cmd_clean;
mod cmd_env;
mod cmd_install;
mod cmd_list_local;
mod cmd_list_remote;
mod cmd_uninstall;
mod cmd_use;

pub(crate) trait Run {
    fn run(&self) -> anyhow::Result<()>;
}

#[derive(Parser)]
#[clap(name = "nvm", author, version, about = "Nodejs Version Manager")]
enum Cli {
    /// List all installed nodejs
    #[clap(visible_alias = "ls")]
    List(ListLocalCommand),

    /// List remote, by default only lts
    #[clap(visible_alias = "lr")]
    ListRemote(ListRemoteCommand),

    /// Use some version
    #[clap(visible_alias = "set")]
    Use(UseCommand),

    /// Install some version
    #[clap(visible_aliases =["i", "add"])]
    Install(InstallCommand),

    /// Uninstall some version
    #[clap(visible_alias = "rm")]
    Uninstall(UninstallCommand),

    /// Print env
    Env(EnvCommand),

    /// Clean cache
    Clean(CleanCommand),
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli {
        Cli::List(cmd) => cmd.run(),
        Cli::ListRemote(cmd) => cmd.run(),
        Cli::Use(cmd) => cmd.run(),
        Cli::Install(cmd) => cmd.run(),
        Cli::Uninstall(cmd) => cmd.run(),
        Cli::Env(cmd) => cmd.run(),
        Cli::Clean(cmd) => cmd.run(),
    }
}
