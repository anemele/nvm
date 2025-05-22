use colored::Colorize;
use nvm_core::local;

pub(super) fn run() -> anyhow::Result<()> {
    let local_versions = local::query()?;
    // dbg!(&local_versions);

    if local_versions.versions.is_empty() {
        println!("Nothing available, install first.");
        return Ok(());
    }

    // See issue: https://github.com/colored-rs/colored/issues/76#issuecomment-616869300
    // and the solution: https://docs.rs/colored/1.9.3/x86_64-pc-windows-msvc/colored/control/fn.set_virtual_terminal.html
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    for v in local_versions.versions {
        if v == local_versions.current {
            println!("* {}", v.green())
        } else {
            println!("  {}", v)
        }
    }

    Ok(())
}
