extern crate walkdir;

use std::fs;
use std::io;
use walkdir::WalkDir;

pub fn find_tfvars_files(current_dir: &std::path::Path) -> Result<Vec<String>, io::Error> {
    let mut results: Vec<String> = Vec::new();
    for entry in WalkDir::new(current_dir.clone()) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(file_name) = entry.path().file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if name_str.ends_with(".tfvars") || name_str.ends_with(".tfvars.json") {
                        println!("{:?}", entry);
                        let entry_path =
                            fs::canonicalize(entry.path().display().to_string()).unwrap();
                        let relative_path = entry_path.strip_prefix(&current_dir).unwrap();
                        results.push(relative_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    Ok(results)
}
