use std::{fs, path::{Path, PathBuf}, time::UNIX_EPOCH};

use freedesktop_icons::lookup;

pub fn last_modified(path: &Path) -> Option<u64> {
    let metadata = fs::metadata(path).ok()?;
    let modified = metadata.modified().ok()?;
    let duration = modified.duration_since(UNIX_EPOCH).ok()?;
    Some(duration.as_secs())
}

pub fn get_icon_path(icon_name: &str) -> Option<PathBuf> {
    // Search direct in breeze for apps
    for size in [32, 48, 64] {
        let direct_path = PathBuf::from(format!(
            "/usr/share/icons/breeze/apps/{}/{}.svg",
            size, icon_name
        ));
        if direct_path.exists() {
            return Some(direct_path);
        }
    }

    // Then specific themes
    for theme in ["Breeze", "Papirus", "Adwaita"] {
        for size in [64, 48, 32] {
            if let Some(path) = lookup(icon_name)
                .with_theme(theme)
                .with_size(size)
                .find() {
                return Some(path);
            }
        }
    }
    
    // Try symbolic
    let symbolic = format!("{}-symbolic", icon_name);
    if let Some(path) = lookup(&symbolic).with_size(32).find() {
        return Some(path);
    }
    // Generic fallback
    lookup("application-x-executable").with_size(48).find()
}