extern crate walkdir;

///use std::process::Command;
use cli::parse_args;
use prompt::select_tfvars_files;
use std::env;
use std::io;
use vars::find_tfvars_files;

// tfam workspace clean
// tfam apply --interactive --concurrent -tf-var=toto.tfvars -tf-var toto.tfvars
pub mod cli;
pub mod prompt;
pub mod vars;

fn main() -> io::Result<()> {
    let mut args = parse_args();
    println!("{:?}", args);
    if args.interactive {
        let current_dir = env::current_dir()?;
        let results = find_tfvars_files(&current_dir)?;
        let files = select_tfvars_files(results);
        match files {
            Ok(mut v) => args.varfiles.append(&mut v),
            Err(e) => println!("Prompt selection failed: {e:?}"),
        }
    }
    args.varfiles.sort();
    println!("{:?}", args.varfiles);

    //    match select_tfvars_files(results) {
    //        Some(selected_indices) => {
    //            println!("Selected files:");
    //            for index in selected_indices {
    //                println!("  {}", results[index]);
    //            }
    //        }
    //        None => {
    //            println!("Prompt was canceled or exited without selection.");
    //        }
    //    }

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
