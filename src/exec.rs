use std::env;
use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::process::ExitStatus;

//use std::thread;

use crate::cli::Commands;
use crate::workspace::get_workspace;

pub fn exec(args: &Vec<String>, cmd: &Commands) -> ExitStatus {
    let executable = env::var("TFAM_EXE").unwrap_or_else(|_| "terraform".to_string());
    let mut exe = Command::new(&executable);
    let exe = exe.args(args);
    let mut last_error = ExitStatus::from_raw(0);

    match cmd.varfiles.is_empty() {
        true => {
            println!("{} {}", executable, args.join(" "));
            exe.status()
                .expect(&format!("failed to execute {}", executable))
        }
        false => match cmd.concurrent {
            true => exe
                .status()
                .expect(&format!("failed to execute {}", executable)),
            false => {
                for f in &cmd.varfiles {
                    let workspace = get_workspace(f, &cmd.workspaceformat);
                    println!(
                        "TF_WORKSPACE={} {} {} -var-file {:?}",
                        workspace,
                        executable,
                        args.join(" "),
                        f
                    );
                    let status_result = exe
                        .env("TF_WORKSPACE", workspace)
                        .arg("-var-file")
                        .arg(f)
                        .status()
                        .expect(&format!("failed to execute {}", executable));
                    if !status_result.success() {
                        last_error = status_result;
                    }
                }
                last_error
            }
        },
    }
}
