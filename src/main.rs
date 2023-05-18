extern crate walkdir;

use cli::print_usage;
use cli::Commands;
use exec::exec;
use prompt::select_tfvars_files;
use std::env;
use std::process::exit;
use vars::find_tfvars_files;

pub mod cli;
pub mod exec;
pub mod prompt;
pub mod vars;
pub mod workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect();
    let mut cmd = Commands::parse_commands(args);
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
    let exit_status = exec(&cmd.tfargs, &cmd);
    exit(exit_status.code().unwrap_or(1));
}
