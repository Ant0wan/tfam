extern crate walkdir;

use inquire::formatter::MultiOptionFormatter;
use inquire::MultiSelect;
use std::env;
use std::fs;
use std::io;
use std::thread;
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
    let formatter: MultiOptionFormatter<String> = &|a| format!("{} tfvars files", a.len());
    let ans = MultiSelect::new("Select tfvars:", results)
        .with_formatter(formatter)
        .prompt();

    match ans {
        Ok(_) => {
            for element in ans.unwrap() {
                thread::spawn(move || {
                    println!("terraform plan -var-file={:?}", element);
                });
            }
        }
        Err(_) => println!("The .tfvars list could not be processed"),
    }
    // Sleep to allow spawned threads to finish before program exits
    thread::sleep(std::time::Duration::from_millis(10));
    Ok(())
}
