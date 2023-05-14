extern crate walkdir;

///use std::process::Command;
use cli::{parse_commands, print_usage};
use exec::exec;
use prompt::select_tfvars_files;
use std::env;
use std::process::exit;
use vars::find_tfvars_files;

// tfam workspace clean// not yet implemented
// tfam -interactive -var-file=toto=ok -var-file toto2 -interactive -concurrent plan -destroy
// tfam plan -interactive -workspace-format '$1_$2_$3'
pub mod cli;
pub mod exec;
pub mod prompt;
pub mod vars;
pub mod workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (args, mut cmd) = parse_commands();
    if cmd.help {
        print_usage();
    }
    if cmd.interactive {
        let current_dir = env::current_dir()?;
        let mut results = find_tfvars_files(&current_dir)?;
        cmd.varfiles.append(&mut results);
        cmd.varfiles = select_tfvars_files(cmd.varfiles).unwrap();
    }
    cmd.varfiles.sort();
    let exit_status = exec(&args, &cmd);
    exit(exit_status.code().unwrap_or(1));
}
