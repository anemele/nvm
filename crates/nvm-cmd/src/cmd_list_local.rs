use colored::Colorize;
use nvm_core::local::query_local;
use nvm_core::utils::get_paths;

pub fn exec() -> anyhow::Result<()> {
    let paths = get_paths()?;

    let local_versions = query_local(&paths.all)?;
    // dbg!(&local_versions);

    if local_versions.versions.len() == 0 {
        println!("no available. install first.");
        return Ok(());
    }

    // See issue: https://github.com/colored-rs/colored/issues/76#issuecomment-616869300
    // and the solution: https://docs.rs/colored/1.9.3/x86_64-pc-windows-msvc/colored/control/fn.set_virtual_terminal.html
    #[cfg(target_family = "windows")]
    {
        colored::control::set_virtual_terminal(true).unwrap();
    }

    for v in local_versions.versions {
        if v == local_versions.current {
            println!("{}", format!("* {}", v).green())
        } else {
            println!("  {}", v)
        }
    }

    Ok(())
}
