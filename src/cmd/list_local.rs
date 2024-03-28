use crate::local::query_local;
use crate::utils::get_path;
use colored::Colorize;

pub fn exec() {
    let Some((all, bin, _)) = get_path() else {
        return;
    };

    let Some(local_versions) = query_local(&all, &bin) else {
        return;
    };

    if local_versions.versions.len() == 0 {
        println!("no available. install first.");
        return;
    }

    // See issue: https://github.com/colored-rs/colored/issues/76#issuecomment-616869300
    // and the solution: https://docs.rs/colored/1.9.3/x86_64-pc-windows-msvc/colored/control/fn.set_virtual_terminal.html
    #[cfg(target_family = "windows")]
    {
        use colored::control::set_virtual_terminal;
        set_virtual_terminal(true).unwrap();
    }

    for v in local_versions.versions {
        if v == local_versions.current {
            println!("* {}", v.green())
        } else {
            println!("  {}", v)
        }
    }
}
