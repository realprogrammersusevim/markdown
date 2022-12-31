use std::fs;
use std::io::Error;
use std::path::Path;
use walkdir;

/// Create a list of all the file in a directory and all subdirectories
pub fn get_files(path: String, only_markdown: bool) -> Vec<String> {
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if only_markdown {
                if entry.file_name().to_str().unwrap().ends_with(".md") {
                    files.push(entry.path().to_str().unwrap().to_string());
                }
            } else {
                files.push(entry.path().to_str().unwrap().to_string());
            }
        }
    }
    files
}

pub fn file_name_from_path(file_path: &str) {
    Path::new(file_path).file_name().unwrap().to_str().unwrap();
}

pub fn rename_file(old_path: String, new_path: String) -> Result<(), Error> {
    fs::rename(old_path, new_path)
}
