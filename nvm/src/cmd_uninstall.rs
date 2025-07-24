use colored::Colorize;
use dialoguer::MultiSelect;
use dialoguer::theme::ColorfulTheme;
use nvm_core::local;
use nvm_core::semver;
use nvm_core::utils;
use std::fs;

pub(super) fn run(versions: Vec<String>) -> anyhow::Result<()> {
    let local_versions = local::query()?;

    let versions: Vec<String> = if versions.is_empty() {
        let sels = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select versions to uninstall")
            .items(&local_versions.versions)
            .interact()?;
        if sels.is_empty() {
            println!("Nothing selected");
            return Ok(());
        }
        sels.iter()
            .map(|&i| local_versions.versions[i].clone())
            .collect()
    } else {
        let (map, _) = semver::map_versions(&local_versions.versions);
        versions
            .iter()
            .map(|v| match map.get(v) {
                Some(s) => s.to_string(),
                None => v.to_string(),
            })
            .collect()
    };

    for version in versions {
        let paths = utils::get_paths()?;
        let want = paths.home.join(format!("v{version}"));
        if want.is_dir() {
            if fs::remove_dir_all(want).is_ok() {
                println!("Removed: {version}");
                if version == local_versions.current {
                    println!("{}: remove current version {}", "WARNING".yellow(), version);
                    let _ = fs::remove_dir(paths.current);
                }
            } else {
                eprintln!("Failed to remove: {version}")
            }
        } else {
            println!("Not found: {version}")
        }
    }

    Ok(())
}
