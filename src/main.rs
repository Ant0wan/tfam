extern crate dialoguer;
extern crate walkdir;

use dialoguer::MultiSelect;
use std::env;
use std::fs;
use std::io::{self, stdin, stdout, Write};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let mut results: Vec<String> = Vec::new();
    let current_dir = env::current_dir()?;
    for entry in WalkDir::new(current_dir.clone()) {
        let entry = entry?;
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map(|e| e == "tfvars")
                .unwrap_or(false)
        {
            let entry_path = fs::canonicalize(entry.path().display().to_string()).unwrap();
            let relative_path = entry_path.strip_prefix(&current_dir).unwrap();
            results.push(relative_path.to_string_lossy().to_string());
        }
    }

    results.sort();
    println!("{:?}", results); //DEBUG
    let selected_options = MultiSelect::new()
        .with_prompt("Select options (Press Space to select, Enter to confirm)")
        .items(&results)
        .interact()
        .unwrap();
    println!("You selected: {:?}", selected_options);

    Ok(())
}
