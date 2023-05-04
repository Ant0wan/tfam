extern crate walkdir;

use inquire::formatter::MultiOptionFormatter;
use inquire::MultiSelect;
use std::env;
use std::env;
use std::fs;
use std::io;
use std::process::Command;
use walkdir::WalkDir;


// tfam workspace clean
// tfam apply --interactive --concurrent -tf-var=toto.tfvars -tf-var toto.tfvars

pub fn print_usage() {
    println!("Usage: tfam [options] [action [arguments]]");
    println!("\nOptions:");
    println!("-h/--help -- print usage message and exit");
    println!("-i/--interactive -- execute with interactive prompt to select tfvars before executing terraform");
}

pub struct Args {
    pub help: bool,
    pub interactive: bool,
    pub concurrent: bool,
    pub varfiles: Vec<String>,
    pub action: Option<String>,
    pub arguments: Vec<String>,
}

impl Args {
    fn new() -> Self {
        Args {
            help: false,
            interactive: false,
            concurrent: false,
            varfiles: Vec::new(),
            action: Option,
            arguments: Vec::new(),
        }
    }
}

pub fn parse_args() -> Args {
    let mut args = Args::new();

    let mut args_iter = env::args().skip(1);
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                args.help = true;
            }
            "-i" | "--interactive" => {
                args.interactive = true; // var-files in cli only
            }
            "-c" | "--concurrent" => {
                args.concurrent = true; // false by default if -i specified
            }
            "-var-file" => {
                if let Some(var_file) = args_iter.next() {
                    args.varfiles.push(var_file);
                } else {
                    // If the next argument is missing, try to parse the argument itself as a var file
                    if let Some(var_file) = arg_iter.next() {
                        args.varfiles.push(var_file);
                    } else {
                        panic!("missing var file");
                    }
                }
            }
            _ => {
                if args.action.is_none() {
                    args.action = Some(arg);
                } else {
                    args.arguments.push(arg);
                }
            }
        }
    }

    args
}

fn main() -> io::Result<()> {
    let args = parse_args();
    if args.help {
        print_usage();
    }

    //    let args: Vec<String> = env::args().skip(1).collect();

//    if args.len() < 1 {
//        let status = Command::new("terraform").status();
//        return Ok(());
//    }
//
//    if args[0] == "init" {
//        let status = Command::new("terraform").args(args.clone()).status();
//        return Ok(());
//    }
//
//    let mut results: Vec<String> = Vec::new();
//    let current_dir = env::current_dir()?;
//    for entry in WalkDir::new(current_dir.clone()) {
//        let entry = entry?;
//        if entry.file_type().is_file()
//            && (entry
//                .path()
//                .extension()
//                .map(|e| e == "tfvars")
//                .unwrap_or(false)
//                || entry
//                    .path()
//                    .extension()
//                    .map(|e| e == "tfvars.json")
//                    .unwrap_or(false))
//        {
//            let entry_path = fs::canonicalize(entry.path().display().to_string()).unwrap();
//            let relative_path = entry_path.strip_prefix(&current_dir).unwrap();
//            results.push(relative_path.to_string_lossy().to_string());
//        }
//    }
//
//    if args.len() > 1 && args[1] == "-var-file" {
//        // test get var file, add it to the list but remove it from cli
//        results.push(args[2].clone());
//    }
//
//    results.sort();
//    let formatter: MultiOptionFormatter<String> = &|a| format!("{} tfvars files", a.len());
//    let ans = MultiSelect::new("Select tfvars:", results)
//        .with_formatter(formatter)
//        .prompt();
//
//    match ans {
//        Ok(_) => {
//            for element in ans.unwrap() {
//                println!("terraform {:?} -var-file {}", args.clone(), element);
//                let status = Command::new("terraform")
//                    .args(args.clone())
//                    .arg("-var-file")
//                    .arg(element)
//                    .status();
//            }
//        }
//        Err(_) => println!("The .tfvars list could not be processed"),
//    }
    Ok(())
}
