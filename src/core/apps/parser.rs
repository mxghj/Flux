use std::{fs, path::PathBuf};

// use freedesktop_entry_parser::parse_entry;

pub fn parse_data() {
    println!("Parsing data...");
    let mut desktops_paths: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(PathBuf::from("/usr/share/applications")) // Verify if read_dir don't get any errors
    {
        desktops_paths.extend(entries.flatten().map(|e| e.path())); // Push in vector all files that is in system_dir
    }
    println!("Desktops: {:?}", desktops_paths);
}

