use std::{fs, path::PathBuf, vec};

use crate::core::apps::utils::flatpak_apps;

pub fn scan_desktop_files() -> Vec<PathBuf> {
    let mut desktops_paths: Vec<PathBuf> = Vec::new();

    for entry in vec!(fs::read_dir(dirs::home_dir().unwrap().join(".local/share/applications")), fs::read_dir(PathBuf::from("/usr/share/applications"))) // Get the apps in system apps dir and in user apps dir
    {
        if entry.is_ok() {
            desktops_paths.extend(entry.unwrap().flatten().map(|e| e.path()));
            // If entry don't get any errors push in desktops_paths all paths will get from system and user apps dir
        }
    }
    desktops_paths.extend(flatpak_apps().unwrap_or_default());
    desktops_paths
}
