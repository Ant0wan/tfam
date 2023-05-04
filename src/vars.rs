extern crate walkdir;

use inquire::formatter::MultiOptionFormatter;
use inquire::MultiSelect;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::process::Command;
use walkdir::WalkDir;


{
    let mut results: Vec<String> = Vec::new();
    let current_dir = env::current_dir()?;
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

    results.sort();
}
