extern crate walkdir;

///use std::process::Command;
use cli::{parse_commands, print_usage};
use prompt::select_tfvars_files;
use std::env;
use std::io;
use vars::find_tfvars_files;

// tfam workspace clean
// tfam apply -interactive -concurrent -tf-var=toto.tfvars -tf-var toto.tfvars
pub mod cli;
pub mod prompt;
pub mod vars;

fn main() -> io::Result<()> {
    let (mut args, mut cmd) = parse_commands();
    if cmd.help {
        print_usage();
    }
    println!("Commands: {:?}", cmd);
    println!("Arguments: {:?}", args);
    cmd.varfiles.sort();
    //  if args.interactive {
    //      let current_dir = env::current_dir()?;
    //      let mut results = find_tfvars_files(&current_dir)?;
    //      results.append(&mut files);
    //      files = select_tfvars_files(results).unwrap();
    //  }
    //  println!("{:?}", files);

    //            for element in ans.unwrap() {
    //                println!("terraform {:?} -var-file {}", args.clone(), element);
    //                let status = Command::new("terraform")
    //                    .args(args.clone())
    //                    .arg("-var-file")
    //                    .arg(element)
    //                    .status();
    //    }
    Ok(())
}
