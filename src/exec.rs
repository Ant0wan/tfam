use std::env;
use std::process::Command;
use std::process::ExitStatus;
use std::thread;

use crate::cli::Commands;
use crate::workspace::get_workspace;

pub fn execute(args: &Vec<String>, cmd: &Commands) {
    let executable = match env::var("TFAM_EXE") {
        Ok(exe) => exe,
        Err(_) => "terraform".to_string(),
    };
    let mut exe = Command::new(executable.clone());
    let exe = exe.args(args);
    let _exit_status = match cmd.varfiles.is_empty() {
        true => {
            println!("{} {}", executable, args.join(" "));
            exe.status();
        }
        false => {
            for f in &cmd.varfiles {
                let workspace = get_workspace(f, &cmd.workspaceformat);
                println!(
                    "TF_WORKSPACE={} {} {} -var-file={:?}",
                    workspace,
                    executable,
                    args.join(" "),
                    f
                );
                exe.env("TF_WORKSPACE", workspace)
                    .arg("-var-file")
                    .arg(f)
                    .status();
            }
        }
    };
}
