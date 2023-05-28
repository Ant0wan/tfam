extern crate walkdir;

use cli::print_usage;
use cli::Commands;
use exec::exec;
use prompt::select_tfvars_files;
use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::process::ExitStatus;
use std::sync::Arc;
use std::sync::Mutex;
use vars::find_tfvars_files;

pub mod cli;
pub mod exec;
pub mod prompt;
pub mod vars;
pub mod workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut cmd: Commands = Commands::parse_commands(args);
    if cmd.help {
        print_usage();
    }
    if cmd.interactive {
        let current_dir: PathBuf = env::current_dir()?;
        let mut results: Vec<String> = find_tfvars_files(&current_dir)?;
        cmd.varfiles.append(&mut results);
        cmd.varfiles = select_tfvars_files(cmd.varfiles).unwrap();
    }
    cmd.varfiles.sort();
    let cmd_arc: Arc<Mutex<Commands>> = Arc::new(Mutex::new(cmd));
    let exit_status: ExitStatus = exec(Arc::clone(&cmd_arc));
    exit(exit_status.code().unwrap_or(1));
}
