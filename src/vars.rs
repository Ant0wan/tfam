extern crate walkdir;

use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

pub fn find_tfvars_files(current_dir: &std::path::Path) -> Result<Vec<String>, io::Error> {
    let mut results: Vec<String> = Vec::new();
    for entry in WalkDir::new(current_dir) {
        let entry: DirEntry = entry?;
        if entry.file_type().is_file() {
            if let Some(file_name) = entry.path().file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.ends_with(".tfvars") || name_str.ends_with(".tfvars.json") {
                        let entry_path: PathBuf =
                            fs::canonicalize(entry.path().display().to_string()).unwrap();
                        let relative_path: &Path = entry_path.strip_prefix(current_dir).unwrap();
                        results.push(relative_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    Ok(results)
}
