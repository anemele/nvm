use crate::core::{get_path, install, list_remote};
use clap::Parser;
use std::{
    fs,
    process::{Command, Stdio},
};

#[derive(Parser)]
#[clap(
name = "nvm",
author="Jason Swift <anemele@outlook.com>",
version,
about = "Nodejs Version Manager",
long_about=None,
)]
pub enum Cli {
    /// List all installed nodejs
    #[clap(alias = "ls")]
    List,
    /// List remote
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
        Cli::List => {
            if let Some((all, _bin, _)) = get_path() {
                println!("all available:");
                if let Ok(rd) = fs::read_dir(all) {
                    for de in rd {
                        if let Ok(dir) = de {
                            println!("  {}", dir.file_name().to_str().unwrap())
                        }
                    }
                }
                // println!("* {}", bin.file_name().unwrap().to_str().unwrap())
            } else {
                println!("no available.")
            };
        }
        Cli::ListRemote => list_remote(),
        Cli::Use { version } => {
            if let Some((all, bin, _)) = get_path() {
                let want = all.join(&format!("v{}", version));
                if !want.exists() {
                    println!("not found: {}", version);
                    return;
                }

                if bin.exists() {
                    let _ = fs::remove_dir(&bin);
                }
                let _ = Command::new("cmd")
                    .args([
                        "/c",
                        "mklink",
                        "/j",
                        bin.to_str().unwrap(),
                        want.to_str().unwrap(),
                    ])
                    .stdout(Stdio::null())
                    .status();
            }
        }
        Cli::Install { version } => {
            install(&version);
        }
        Cli::Uninstall { version } => {
            if let Some((all, _, _)) = get_path() {
                let want = all.join(&version);
                if want.exists() {
                    let _ = fs::remove_dir_all(want);
                    return;
                }
            }

            println!("not found: {}", version)
        }
    }
}
