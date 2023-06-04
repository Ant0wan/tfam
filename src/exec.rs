use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::process::ExitStatus;
use std::sync::Arc;
use std::thread;

use crate::cli::Commands;
use crate::workspace::get;

fn process_file(cmd: &Commands, file: &String) -> ExitStatus {
    let workspace: String = get(file, &cmd.workspaceformat);
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
        .env("TF_VAR_FILE", file)
        .status()
        .unwrap_or_else(|_| panic!("failed to execute {}", cmd.bin))
}

/// # Panics
///
/// Will panic if cmd.lock does not exists
#[must_use]
pub fn exec(cmd: &Arc<Commands>) -> ExitStatus {
    let mut last_error: ExitStatus = ExitStatus::from_raw(0);

    if cmd.varfiles.is_empty() {
        println!("{} {}", cmd.bin, cmd.tfargs.join(" "));
        Command::new(&cmd.bin)
            .args(&cmd.tfargs)
            .status()
            .unwrap_or_else(|_| panic!("failed to execute {}", cmd.bin))
    } else if cmd.concurrent {
        let handles: Vec<_> = cmd
            .varfiles
            .iter()
            .map(|f: &String| {
                let cmd_clone = cmd.clone();
                let file_clone: String = f.clone();
                thread::spawn(move || process_file(&cmd_clone, &file_clone))
            })
            .collect();
        for handle in handles {
            let status_result: ExitStatus = handle.join().unwrap();
            if !status_result.success() {
                last_error = status_result;
            }
        }
        last_error
    } else {
        for file in &cmd.varfiles {
            let status_result: ExitStatus = process_file(cmd, file);
            if !status_result.success() {
                last_error = status_result;
            }
        }
        last_error
    }
}
