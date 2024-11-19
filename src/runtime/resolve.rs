use std::fs::read_dir;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
const PATH_SEPARATOR: char = ';';

#[cfg(target_os = "linux")]
const PATH_SEPARATOR: char = ':';

pub fn search_program_in_path(path_text: &str, file_name: &str) -> Option<PathBuf> {
    for path in path_text.split(';') {
        if path == "" {
            continue
        }

        let dir_entries = match read_dir(Path::new(path)) {
            Ok(entries) => entries,
            Err(_) => continue // ignore invalid directories
        };

        for entry in dir_entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue
            };

            if entry.file_name() == file_name {
                return Some(entry.path())
            }
        }
    }

    None
}