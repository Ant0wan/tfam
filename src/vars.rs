extern crate walkdir;

use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

/// Find `TFVars` files in the current directory.
///
/// # Errors
///
/// Returns an error if there is an issue with finding the `TFVars` files, such as a failure to read the directory or file I/O errors.
///
/// # Arguments
///
/// * `current_dir` - A reference to the current directory path.
///
/// # Returns
///
/// A `Result` containing a vector of found `TFVars` files or an `io::Error`.
///
/// # Panics
///
/// Will panic if error in getting relative path
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
