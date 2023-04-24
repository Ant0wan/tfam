extern crate walkdir;

use inquire::formatter::MultiOptionFormatter;
use inquire::MultiSelect;
use std::env;
use std::fs;
use std::io;
use std::process::Command;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        let status = Command::new("terraform").status();
        return Ok(());
    }

    if args[0] == "init" {
        let status = Command::new("terraform").args(args.clone()).status();
        return Ok(());
    }

    let mut results: Vec<String> = Vec::new();
    let current_dir = env::current_dir()?;
    for entry in WalkDir::new(current_dir.clone()) {
        let entry = entry?;
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .map(|e| e == "tfvars" || e == "tfvars.json")
                .unwrap_or(false)
        {
            let entry_path = fs::canonicalize(entry.path().display().to_string()).unwrap();
            let relative_path = entry_path.strip_prefix(&current_dir).unwrap();
            results.push(relative_path.to_string_lossy().to_string());
        }
    }

    if args.len() > 1 && args[1] == "-var-file" {
        // test get var file, add it to the list but remove it from cli
        results.push(args[2].clone());
    }

    results.sort();
    let formatter: MultiOptionFormatter<String> = &|a| format!("{} tfvars files", a.len());
    let ans = MultiSelect::new("Select tfvars:", results)
        .with_formatter(formatter)
        .prompt();

    match ans {
        Ok(_) => {
            for element in ans.unwrap() {
                println!("terraform {:?} -var-file {}", args.clone(), element);
                let status = Command::new("terraform")
                    .args(args.clone())
                    .arg("-var-file")
                    .arg(element)
                    .status();
            }
        }
        Err(_) => println!("The .tfvars list could not be processed"),
    }
    Ok(())
}
