use std::fs;
use std::io::Error;
use walkdir;

/// Create a list of all the file in a directory and all subdirectories
pub fn get_files(path: String) -> Vec<String> {
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            files.push(entry.path().to_string_lossy().into_owned());
        }
    }
    files
}

pub fn rename_file(old_path: String, new_path: String) -> Result<(), Error> {
    fs::rename(old_path, new_path)
}
