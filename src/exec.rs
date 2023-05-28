use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::process::ExitStatus;
use std::thread;

use crate::cli::Commands;
use crate::workspace::get_workspace;

fn process_file(cmd: &Commands, file: &String) -> ExitStatus {
    let workspace: String = get_workspace(file, &cmd.workspaceformat);
    println!(
        "TF_WORKSPACE={} {} {} -var-file {:?}",
        workspace,
        cmd.bin,
        cmd.tfargs.join(" "),
        file
    );
    Command::new(&cmd.bin)
        .args(&cmd.tfargs)
        .env("TF_WORKSPACE", workspace)
        .arg("-var-file")
        .arg(file)
        .status()
        .expect(&format!("failed to execute {}", cmd.bin))
}

pub fn exec(cmd: &Commands) -> ExitStatus {
    let mut last_error: ExitStatus = ExitStatus::from_raw(0);

    match cmd.varfiles.is_empty() {
        true => {
            println!("{} {}", cmd.bin, cmd.tfargs.join(" "));
            Command::new(&cmd.bin)
                .args(&cmd.tfargs)
                .status()
                .expect(&format!("failed to execute {}", cmd.bin))
        }
        false => match cmd.concurrent {
            true => {
                let handles: Vec<_> = cmd
                    .varfiles
                    .iter()
                    .map(|f: &String| thread::spawn(move || process_file(cmd, f)))
                    .collect();
                for handle in handles {
                    let status_result: ExitStatus = handle.join().unwrap();
                    if !status_result.success() {
                        last_error = status_result;
                    }
                }
                last_error
            }
            false => {
                for file in &cmd.varfiles {
                    let status_result: ExitStatus = process_file(cmd, file);
                    if !status_result.success() {
                        last_error = status_result;
                    }
                }
                last_error
            }
        },
    }
}
