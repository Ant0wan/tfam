extern crate walkdir;

use inquire::formatter::MultiOptionFormatter;
use inquire::MultiSelect;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::process::Command;
use walkdir::WalkDir;

// tfam workspace clean
// tfam apply --interactive --concurrent -tf-var=toto.tfvars -tf-var toto.tfvars

fn main() -> io::Result<()> {
    let args = parse_args();
    let current_dir = env::current_dir()?;
    let results = find_tfvars_files(&current_dir)?;

    match select_tfvars_files(results) {
        Some(selected_indices) => {
            println!("Selected files:");
            for index in selected_indices {
                println!("  {}", results[index]);
            }
        }
        None => {
            println!("Prompt was canceled or exited without selection.");
        }
    }

    //   let ans = MultiSelect::new("Select tfvars:", results)

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
    //println!("{:?}", args);
    Ok(())
}
