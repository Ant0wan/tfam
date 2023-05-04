extern crate walkdir;

use std::fs;
use std::io;
use walkdir::WalkDir;

pub fn find_tfvars_files(current_dir: &std::path::Path) -> Result<Vec<String>, io::Error> {
    let mut results: Vec<String> = Vec::new();
    for entry in WalkDir::new(current_dir.clone()) {
        let entry = entry?;
        if entry.file_type().is_file()
            && (entry
                .path()
                .extension()
                .map(|e| e == "tfvars")
                .unwrap_or(false)
                || entry
                    .path()
                    .extension()
                    .map(|e| e == "tfvars.json")
                    .unwrap_or(false))
        {
            let entry_path = fs::canonicalize(entry.path().display().to_string()).unwrap();
            let relative_path = entry_path.strip_prefix(&current_dir).unwrap();
            results.push(relative_path.to_string_lossy().to_string());
        }
    }

    Ok(results)
}
